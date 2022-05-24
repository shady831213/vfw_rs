extern crate alloc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
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
    fn read_line(&mut self) -> Result<String> {
        let mut b = vec![];
        loop {
            let mut byte = [0u8; 1];
            nb::block!(self.read_exact(&mut byte))?;
            if byte[0] == b'\n' || byte[0] == b'\0' {
                break;
            }
            b.push(byte[0])
        }
        Ok(core::str::from_utf8(&b)
            .map_err(|e| IoError::BadString(e))?
            .to_string())
    }
}
