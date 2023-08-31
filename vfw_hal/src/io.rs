#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[derive(Debug)]
pub enum IoError {
    UnExpectedEOF,
    BadString(core::str::Utf8Error),
}
pub type Error = nb::Error<IoError>;
pub type Result<T> = nb::Result<T, IoError>;

pub trait Write {
    fn write(&mut self, data: &[u8]) -> Result<usize>;
    fn write_all(&mut self, data: &[u8]) -> Result<()> {
        let mut cnt = 0;
        while cnt != data.len() {
            cnt += nb::block!(self.write(&data[cnt..]))?;
        }
        Ok(())
    }
}

pub trait Read {
    fn read(&mut self, data: &mut [u8]) -> Result<usize>;
    fn read_exact(&mut self, data: &mut [u8]) -> Result<()> {
        let mut cnt = 0;
        while cnt != data.len() {
            cnt += nb::block!(self.read(&mut data[cnt..]))?;
        }
        Ok(())
    }
    #[cfg(feature = "alloc")]
    fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> Result<usize> {
        let len = buf.len();
        loop {
            let mut b = [0u8; 1];
            nb::block!(self.read_exact(&mut b))?;
            if b[0] == byte {
                break;
            }
            buf.push(b[0])
        }
        Ok(buf.len() - len)
    }
    #[cfg(feature = "alloc")]
    fn read_line(&mut self, buf: &mut String) -> Result<usize> {
        unsafe {
            let len = buf.len();
            let b = buf.as_mut_vec();
            match self.read_until(b'\n', b) {
                Ok(l) => match core::str::from_utf8(&b[len..]) {
                    Ok(_) => Ok(l),
                    Err(e) => {
                        b.set_len(len);
                        Err(Error::Other(IoError::BadString(e)))
                    }
                },
                Err(e) => {
                    b.set_len(len);
                    Err(e)
                }
            }
        }
    }
}
