#![allow(dead_code)]
use vfw_primitives::{io_read32, io_write32};

const PLIC_PRIORITY_OFFSET: usize = 0x00000000;
const PLIC_PRIORITY_SHIFT_PER_SOURCE: usize = 2;

const PLIC_PENDING_OFFSET: usize = 0x00001000;

const PLIC_ENABLE_OFFSET: usize = 0x00002000;
const PLIC_ENABLE_SHIFT_PER_TARGET: usize = 7;

const PLIC_THRESHOLD_OFFSET: usize = 0x00200000;
const PLIC_THRESHOLD_SHIFT_PER_TARGET: usize = 12;

const PLIC_CLAIM_OFFSET: usize = 0x00200004;
const PLIC_CLAIM_SHIFT_PER_TARGET: usize = 12;
#[repr(C)]
pub struct Plic {
    base: usize,
    max_pri: usize,
}

impl Plic {
    pub const fn new(base: usize, max_pri: usize) -> Self {
        Self { base, max_pri }
    }

    pub fn set_feature(&self, feature: usize) {
        io_write32!(self.base, feature)
    }
    pub fn feature(&self) -> usize {
        io_read32!(self.base) as usize
    }
    pub fn set_pri(&self, irq: usize, pri: usize) {
        io_write32!(
            self.base + PLIC_PRIORITY_OFFSET + (irq << PLIC_PRIORITY_SHIFT_PER_SOURCE),
            {
                if pri > self.max_pri {
                    self.max_pri
                } else {
                    pri
                }
            }
        )
    }
    pub fn set_threshold(&self, threshold: usize, target_id: usize) {
        io_write32!(
            self.base + PLIC_THRESHOLD_OFFSET + (target_id << PLIC_THRESHOLD_SHIFT_PER_TARGET),
            threshold
        )
    }
    pub fn raise_int(&self, irq: usize) {
        io_write32!(
            self.base + PLIC_PENDING_OFFSET + ((irq >> 5) << 2),
            1 << (irq & 0x1F)
        )
    }
    pub fn pending(&self, irq: usize) -> bool {
        io_read32!(self.base + PLIC_PENDING_OFFSET + ((irq >> 5) << 2)) & (1 << (irq & 0x1F)) != 0
    }
    pub fn enable(&self, irq: usize, target_id: usize) {
        let addr = self.base
            + PLIC_ENABLE_OFFSET
            + (target_id << PLIC_ENABLE_SHIFT_PER_TARGET)
            + ((irq >> 5) << 2);
        let flags = io_read32!(addr);
        io_write32!(addr, flags | (1 << (irq & 0x1f)) as u32)
    }
    pub fn disable(&self, irq: usize, target_id: usize) {
        let addr = (self.base
            + PLIC_ENABLE_OFFSET
            + (target_id << PLIC_ENABLE_SHIFT_PER_TARGET)
            + ((irq >> 5) << 2)) as *mut u32;
        let flags = io_read32!(addr);
        io_write32!(addr, flags & !(1 << (irq & 0x1f)) as u32)
    }
    pub fn claim(&self, target_id: usize) -> usize {
        io_read32!(self.base + PLIC_CLAIM_OFFSET + (target_id << PLIC_CLAIM_SHIFT_PER_TARGET))
            as usize
    }
    pub fn complete(&self, irq: usize, target_id: usize) {
        io_write32!(
            self.base + PLIC_CLAIM_OFFSET + (target_id << PLIC_CLAIM_SHIFT_PER_TARGET),
            irq
        );
    }
}

#[no_mangle]
extern "C" fn plic_max_pri(p: &Plic) -> u32 {
    p.max_pri as u32
}

#[no_mangle]
extern "C" fn plic_set_pri(p: &Plic, irq: u32, pri: u32) {
    p.set_pri(irq as usize, pri as usize)
}

#[no_mangle]
extern "C" fn plic_raise_int(p: &Plic, irq: u32) {
    p.raise_int(irq as usize)
}

#[linkage = "weak"]
#[no_mangle]
extern "C" fn plic_set_threshold(p: &Plic, threshold: u32) {
    p.set_threshold(threshold as usize, crate::hartid())
}

#[linkage = "weak"]
#[no_mangle]
extern "C" fn plic_enable(p: &Plic, irq: u32) {
    p.enable(irq as usize, crate::hartid())
}

#[linkage = "weak"]
#[no_mangle]
extern "C" fn plic_disable(p: &Plic, irq: u32) {
    p.disable(irq as usize, crate::hartid())
}
