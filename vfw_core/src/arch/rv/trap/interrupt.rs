use crate::arch::FlowContext;
use crate::arch::{default_trap_handler, dummy_trap_handler};
use crate::{per_cpu_offset, PER_CPU_LEN};
use riscv::register::mcause::Interrupt;

pub union InterruptVector {
    pub handler: unsafe extern "C" fn(),
    pub reserved: usize,
}

impl InterruptVector {
    pub unsafe fn handle(&self, ctx: &mut FlowContext) {
        if self.reserved == 0 {
            default_trap_handler(ctx);
        } else {
            (self.handler)();
        }
    }

    pub unsafe fn handle_or_dummy(&self) {
        if self.reserved == 0 {
            dummy_trap_handler();
        } else {
            (self.handler)();
        }
    }
}

const INT_VECTOR_LEN: usize = 13;

#[inline(always)]
pub fn interrupt_handler(ctx: &mut FlowContext, cause: Interrupt) {
    unsafe {
        interrupts()[per_cpu_offset()][cause as usize].handle(ctx);
    }
}

#[inline]
fn interrupts() -> &'static mut [[InterruptVector; INT_VECTOR_LEN]] {
    crate::relocation!(mut __INTERRUPTS: [[InterruptVector; INT_VECTOR_LEN]; PER_CPU_LEN])
}

pub unsafe fn register_int_handler(cause: Interrupt, f: unsafe extern "C" fn()) {
    interrupts()[per_cpu_offset()][cause as usize].handler = f;
}

#[link_section = ".cpu.bss"]
static mut __INTERRUPTS: [[InterruptVector; INT_VECTOR_LEN]; PER_CPU_LEN] =
    [const { [const { InterruptVector { reserved: 0 } }; INT_VECTOR_LEN] }; PER_CPU_LEN];
