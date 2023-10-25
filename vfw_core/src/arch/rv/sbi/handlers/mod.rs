mod misaligned_access;
pub use misaligned_access::*;
mod rdtime;
use crate::arch::FlowContext;
pub use rdtime::*;
mod interrupt;
pub use interrupt::*;
mod boot;
pub use boot::*;
mod sbi_call;
pub use sbi_call::*;

#[derive(Debug)]
pub enum SbiHandlerError {
    Unhandled,
}

fn update_reg(ctx: &mut FlowContext, dst: u8, value: usize) {
    match dst {
        1 => ctx.ra = value,
        2 => ctx.sp = value,
        3 => ctx.gp = value,
        4 => ctx.tp = value,
        5..=7 => ctx.t[dst as usize - 5] = value,
        8..=9 => ctx.s[dst as usize - 8] = value,
        10..=17 => ctx.a[dst as usize - 10] = value,
        18..=27 => ctx.s[dst as usize - 18 + 2] = value,
        28..=31 => ctx.t[dst as usize - 28 + 3] = value,
        _ => panic!("invalid target {}", dst),
    }
}
fn get_reg(ctx: &mut FlowContext, src: u8) -> usize {
    match src {
        0 => 0,
        1 => ctx.ra,
        2 => ctx.sp,
        3 => ctx.gp,
        4 => ctx.tp,
        5..=7 => ctx.t[src as usize - 5],
        8..=9 => ctx.s[src as usize - 8],
        10..=17 => ctx.a[src as usize - 10],
        18..=27 => ctx.s[src as usize - 18 + 2],
        28..=31 => ctx.t[src as usize - 28 + 3],
        _ => panic!("invalid src {}", src),
    }
}

#[macro_export]
macro_rules! sbi_trap_handler {
    ($handler:ident) => {
        #[naked]
        extern "C" fn sbi_trap_handler(_regs: &mut crate::FlowContext) {
            unsafe {
                core::arch::asm!("mv sp, tp",
                "j {handler}",
                handler = sym $handler,
                options(noreturn)
                )
            }
        }
    }
}

#[macro_export]
macro_rules! sbi_to_next_stage {
    ($stack_locate:ident, $rust_main:ident) => {
        #[naked]
        unsafe extern "C" fn sbi_to_next_stage(_hart_id: usize, _start_addr: usize, _opaque: usize) -> ! {
            core::arch::asm!(
                "
                    call {locate_stack}
                    call {rust_main}
                    j    {trap}
                ",
                locate_stack = sym $stack_locate,
                rust_main    = sym $rust_main,
                trap         = sym crate::trap_entry,
                options(noreturn),
            )
        }
    }
}
