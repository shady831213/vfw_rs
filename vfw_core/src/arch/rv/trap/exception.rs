use crate::arch::FlowContext;
use crate::TrapVector;
use crate::{per_cpu_offset, PER_CPU_LEN};
pub type ExceptionVector = TrapVector;

#[inline(always)]
pub(super) fn exception_handler(ctx: &mut FlowContext) {
    unsafe { expts()[per_cpu_offset()].handle(ctx) };
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
