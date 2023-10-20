use crate::arch::default_trap_handler;
use crate::arch::FlowContext;
use crate::{per_cpu_offset, PER_CPU_LEN};
pub union ExceptionVector {
    pub handler: crate::TrapHandler,
    pub reserved: usize,
}

impl ExceptionVector {
    #[inline]
    pub unsafe fn handle(&self, ctx: &mut FlowContext) {
        if self.reserved == 0 {
            default_trap_handler(ctx);
        } else {
            (self.handler)(ctx);
        }
    }
}

#[inline(always)]
pub unsafe extern "C" fn exception_handler(ctx: &mut FlowContext) {
    expts()[per_cpu_offset()].handle(ctx);
}

#[inline]
fn expts() -> &'static mut [ExceptionVector] {
    crate::relocation!(mut __EXCEPTIONS: [ExceptionVector; PER_CPU_LEN])
}

pub unsafe fn register_exception_handler(f: crate::TrapHandler) {
    expts()[per_cpu_offset()].handler = f;
}

#[link_section = ".cpu.bss"]
static mut __EXCEPTIONS: [ExceptionVector; PER_CPU_LEN] =
    [const { ExceptionVector { reserved: 0 } }; PER_CPU_LEN];
