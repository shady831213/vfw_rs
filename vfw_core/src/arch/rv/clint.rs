#![allow(dead_code)]
use vfw_primitives::{io_read32, io_read64, io_write32, io_write64};

const MTIME_OFF: usize = 0xbff8;
const TIMER_OFF: usize = 0x4000;

pub struct Clint {
    base: usize,
    access_64: bool,
}

impl Clint {
    pub const fn new(base: usize, access_64: bool) -> Clint {
        Clint { base, access_64 }
    }

    pub fn get_mtime(&self) -> u64 {
        if self.access_64 {
            io_read64!(self.base + MTIME_OFF)
        } else {
            let mut hi = io_read32!(self.base + MTIME_OFF + 4);
            let mut lo = io_read32!(self.base + MTIME_OFF);
            loop {
                let new_hi = io_read32!(self.base + MTIME_OFF + 4);
                if new_hi == hi {
                    break ((new_hi as u64) << 32) | (lo as u64);
                } else {
                    hi = new_hi;
                    lo = io_read32!(self.base + MTIME_OFF);
                }
            }
        }
    }

    pub fn set_timer(&self, hart_id: usize, instant: u64) {
        if self.access_64 {
            io_write64!(self.base + TIMER_OFF + (hart_id << 3), instant)
        } else {
            io_write32!(self.base + TIMER_OFF + (hart_id << 3), instant as u32);
            io_write32!(
                self.base + TIMER_OFF + (hart_id << 3) + 4,
                (instant >> 32) as u32
            );
        }
    }

    pub fn send_soft(&self, hart_id: usize) {
        io_write32!(self.base + (hart_id << 2), 1)
    }

    pub fn read_soft(&self, hart_id: usize) -> bool {
        io_read32!(self.base + (hart_id << 2)) != 0
    }

    pub fn clear_soft(&self, hart_id: usize) {
        io_write32!(self.base + (hart_id << 2), 0)
    }
}
