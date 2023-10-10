use super::default_trap_handler;
use crate::{per_cpu_offset, PER_CPU_LEN};
use fast_trap::FastContext;
pub union ExceptionVector {
    pub handler: unsafe extern "C" fn(
        ctx: FastContext,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
        a6: usize,
        a7: usize,
    ),
    pub reserved: usize,
}

impl ExceptionVector {
    #[inline]
    pub unsafe fn handle(
        &self,
        ctx: FastContext,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
        a6: usize,
        a7: usize,
    ) {
        if self.reserved == 0 {
            default_trap_handler();
        } else {
            (self.handler)(ctx, a1, a2, a3, a4, a5, a6, a7);
        }
    }
}

#[inline(always)]
pub unsafe extern "C" fn exception_handler(
    ctx: FastContext,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
) {
    expts()[per_cpu_offset()].handle(ctx, a1, a2, a3, a4, a5, a6, a7);
}

#[inline]
fn expts() -> &'static mut [ExceptionVector] {
    crate::relocation!(mut __EXCEPTIONS: [ExceptionVector; PER_CPU_LEN])
}

pub unsafe fn register_exception_handler(
    f: unsafe extern "C" fn(
        ctx: FastContext,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
        a6: usize,
        a7: usize,
    ),
) {
    expts()[per_cpu_offset()].handler = f;
}

#[link_section = ".cpu.bss"]
static mut __EXCEPTIONS: [ExceptionVector; PER_CPU_LEN] =
    [const { ExceptionVector { reserved: 0 } }; PER_CPU_LEN];
