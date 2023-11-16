#![allow(dead_code)]
use vfw_primitives::{io_read32, io_write32};

const PLIC_PRIORITY_OFFSET: usize = 0x00000000;

const PLIC_PENDING_OFFSET: usize = 0x00001000;

const PLIC_ENABLE_OFFSET: usize = 0x00002000;

const PLIC_THRESHOLD_OFFSET: usize = 0x00200000;

const PLIC_CLAIM_OFFSET: usize = 0x00200004;

#[repr(C)]
pub struct Plic<
    const PLIC_PRIORITY_SHIFT_PER_SOURCE: usize,
    const PLIC_ENABLE_SHIFT_PER_TARGET: usize,
    const PLIC_THRESHOLD_SHIFT_PER_TARGET: usize,
    const PLIC_CLAIM_SHIFT_PER_TARGET: usize,
> {
    base: usize,
    pub max_pri: usize,
}

impl<
        const PLIC_PRIORITY_SHIFT_PER_SOURCE: usize,
        const PLIC_ENABLE_SHIFT_PER_TARGET: usize,
        const PLIC_THRESHOLD_SHIFT_PER_TARGET: usize,
        const PLIC_CLAIM_SHIFT_PER_TARGET: usize,
    >
    Plic<
        PLIC_PRIORITY_SHIFT_PER_SOURCE,
        PLIC_ENABLE_SHIFT_PER_TARGET,
        PLIC_THRESHOLD_SHIFT_PER_TARGET,
        PLIC_CLAIM_SHIFT_PER_TARGET,
    >
{
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
    pub fn set_threshold(&self, threshold: usize, hart_id: usize) {
        io_write32!(
            self.base + PLIC_THRESHOLD_OFFSET + (hart_id << PLIC_THRESHOLD_SHIFT_PER_TARGET),
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
    pub fn enable(&self, irq: usize, hart_id: usize) {
        let addr = self.base
            + PLIC_ENABLE_OFFSET
            + (hart_id << PLIC_ENABLE_SHIFT_PER_TARGET)
            + ((irq >> 5) << 2);
        let flags = io_read32!(addr);
        io_write32!(addr, flags | (1 << (irq & 0x1f)) as u32)
    }
    pub fn disable(&self, irq: usize, hart_id: usize) {
        let addr = (self.base
            + PLIC_ENABLE_OFFSET
            + (hart_id << PLIC_ENABLE_SHIFT_PER_TARGET)
            + ((irq >> 5) << 2)) as *mut u32;
        let flags = io_read32!(addr);
        io_write32!(addr, flags & !(1 << (irq & 0x1f)) as u32)
    }
    pub fn claim(&self, hart_id: usize) -> usize {
        io_read32!(self.base + PLIC_CLAIM_OFFSET + (hart_id << PLIC_CLAIM_SHIFT_PER_TARGET))
            as usize
    }
    pub fn complete(&self, irq: usize, hart_id: usize) {
        io_write32!(
            self.base + PLIC_CLAIM_OFFSET + (hart_id << PLIC_CLAIM_SHIFT_PER_TARGET),
            irq
        );
    }
}
