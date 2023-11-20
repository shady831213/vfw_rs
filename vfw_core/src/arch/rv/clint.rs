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

    pub fn set_timer(&self, target_id: usize, instant: u64) {
        if self.access_64 {
            io_write64!(self.base + TIMER_OFF + (target_id << 3), instant)
        } else {
            io_write32!(self.base + TIMER_OFF + (target_id << 3), instant as u32);
            io_write32!(
                self.base + TIMER_OFF + (target_id << 3) + 4,
                (instant >> 32) as u32
            );
        }
    }

    pub fn send_soft(&self, target_id: usize) {
        io_write32!(self.base + (target_id << 2), 1)
    }

    pub fn read_soft(&self, target_id: usize) -> bool {
        io_read32!(self.base + (target_id << 2)) != 0
    }

    pub fn clear_soft(&self, target_id: usize) {
        io_write32!(self.base + (target_id << 2), 0)
    }
}

#[no_mangle]
extern "C" fn clint_get_mtime(c: &Clint) -> u64 {
    c.get_mtime()
}

#[linkage = "weak"]
#[no_mangle]
extern "C" fn clint_set_timer(c: &Clint, instant: u64) {
    c.set_timer(crate::hartid(), instant)
}
