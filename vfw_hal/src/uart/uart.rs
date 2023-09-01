use super::regs::*;
use crate::clk::Clk;
use crate::io::{self, Read, Write};
use core::ops::Deref;
use spin::Mutex;
use vfw_primitives::{io_read16, io_read32, io_read8, io_write16, io_write32, io_write8};

#[derive(Debug)]
pub enum UartError {
    BaudTooHigh,
}
pub type UartResult<T> = Result<T, UartError>;
pub trait Uart: Write + Read {
    fn setup(&self) {}
}

pub struct UartBase<C: 'static + Clk, const REG_SHIFT: usize> {
    clk: &'static C,
    base: usize,
}

impl<C: 'static + Clk, const REG_SHIFT: usize> UartBase<C, REG_SHIFT> {
    pub const fn new(clk: &'static C, base: usize) -> Self {
        UartBase { clk, base }
    }

    fn reg_addr(&self, offset: usize) -> usize {
        self.base + (offset << REG_SHIFT)
    }

    pub fn write_reg(&self, offset: usize, data: usize) {
        //TODO: alwyas access in byte should work
        match REG_SHIFT {
            0 => io_write8!(self.reg_addr(offset), data),
            1 => io_write16!(self.reg_addr(offset), data as u8),
            2 => io_write32!(self.reg_addr(offset), data as u8),
            _ => unreachable!(),
        }
    }

    pub fn read_reg(&self, offset: usize) -> usize {
        //TODO: alwyas access in byte should work
        match REG_SHIFT {
            0 => io_read8!(self.reg_addr(offset)) as usize,
            1 => io_read16!(self.reg_addr(offset)) as usize,
            2 => io_read32!(self.reg_addr(offset)) as usize,
            _ => unreachable!(),
        }
    }

    pub fn set_reg(&self, offset: usize, data: usize) {
        self.write_reg(offset, data | self.read_reg(offset))
    }

    pub fn clr_reg(&self, offset: usize, data: usize) {
        self.write_reg(offset, !data & self.read_reg(offset))
    }

    pub fn set_baud_rate(&self, baud_rate: usize) -> UartResult<usize> {
        self.set_reg(UART_LCR, UART_LCR_DLAB);
        //Calculate div roughly first
        let ref_freq = self.clk.calculate();
        if (baud_rate << 4) > ref_freq {
            return Err(UartError::BaudTooHigh);
        }
        let d = ref_freq / (baud_rate << 4);
        let d2 = d + 1;
        //Choose best one
        let freq = d * baud_rate << 4;
        // freq <= ref_freq always
        let diff = ref_freq - freq;
        let freq2 = d2 * baud_rate << 4;
        // freq2 >= ref_freq always
        let diff2 = freq2 - ref_freq;

        let (div, baud_real) = if diff < diff2 {
            (d, ref_freq / (d << 4))
        } else {
            (d2, ref_freq / (d2 << 4))
        };

        self.write_reg(UART_DLL, div);
        self.write_reg(UART_DLM, div >> 8);
        self.clr_reg(UART_LCR, UART_LCR_DLAB);
        Ok(baud_real)
    }

    pub fn put_byte(&self, c: u8) -> io::Result<()> {
        let lsr = self.read_reg(UART_LSR);
        if (lsr & (UART_LSR_THRE | UART_LSR_TEMT)) == 0 {
            Err(io::Error::WouldBlock)
        } else {
            self.write_reg(UART_TX, c as usize);
            Ok(())
        }
    }
    pub fn get_byte(&self) -> io::Result<u8> {
        let lsr = self.read_reg(UART_LSR);
        if (lsr & UART_LSR_DR) == 0 {
            Err(io::Error::WouldBlock)
        } else {
            Ok(self.read_reg(UART_RX) as u8)
        }
    }

    pub fn enable_fifo(&self) {
        //enable fifo
        self.set_reg(UART_FCR, UART_FCR_ENABLE_FIFO);
        //enable programmalbe THRE
        self.set_reg(UART_IER, UART_IER_PTIME);
    }
}

pub struct UartSimple<C: 'static + Clk, const REG_SHIFT: usize> {
    inner: UartBase<C, REG_SHIFT>,
}

impl<C: 'static + Clk, const REG_SHIFT: usize> UartSimple<C, REG_SHIFT> {
    pub const fn new(clk: &'static C, base: usize) -> Self {
        UartSimple {
            inner: UartBase::new(clk, base),
        }
    }
}

impl<C: 'static + Clk, const REG_SHIFT: usize> Write for UartSimple<C, REG_SHIFT> {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        for i in 0..data.len() {
            if self.inner.put_byte(data[i]).is_err() {
                return Ok(i);
            }
        }
        Ok(data.len())
    }
}

impl<C: 'static + Clk, const REG_SHIFT: usize> Read for UartSimple<C, REG_SHIFT> {
    fn read(&mut self, data: &mut [u8]) -> io::Result<usize> {
        for i in 0..data.len() {
            if let Ok(c) = self.inner.get_byte() {
                data[i] = c
            } else {
                return Ok(i);
            }
        }
        Ok(data.len())
    }
}

impl<C: 'static + Clk, const REG_SHIFT: usize> Uart for UartSimple<C, REG_SHIFT> {}

impl<C: 'static + Clk, const REG_SHIFT: usize> Deref for UartSimple<C, REG_SHIFT> {
    type Target = UartBase<C, REG_SHIFT>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct LockedUart<T: Uart>(Mutex<T>);

impl<T: Uart> LockedUart<T> {
    pub const fn new(uart: T) -> Self {
        LockedUart(Mutex::new(uart))
    }
}

impl<T: Uart> Deref for LockedUart<T> {
    type Target = Mutex<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
