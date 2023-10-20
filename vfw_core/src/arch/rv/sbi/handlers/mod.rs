mod misaligned_access;
pub use misaligned_access::*;
mod rdtime;
pub use rdtime::*;
mod sbi_call;
use crate::arch::FlowContext;
pub use sbi_call::*;

use riscv::register::{mcause, mepc, mstatus, mtval};

#[derive(Debug)]
pub enum SbiHandlerError {
    Unhandled,
}

pub fn sbi_handler_panic() {
    panic!(
        "Unhandled exception! mstatus: {:x?}, mcause: {:?}, mepc: {:08x?}, insn:{:x},  mtval: {:08x?}",
        mstatus::read(),
        mcause::read().cause(),
        mepc::read(),
        unsafe {super::access::get_insn(mepc::read())},
        mtval::read(),
    );
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
