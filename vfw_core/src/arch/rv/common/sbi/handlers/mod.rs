mod misaligned_access;
pub use misaligned_access::*;
mod rdtime;
pub use rdtime::*;
mod sbi_call;
pub use sbi_call::*;

use crate::arch::rv::common::trap::TrapFrame;
use riscv::register::{mcause, mepc, mstatus, mtval};

#[derive(Debug)]
pub enum SbiHandlerError {
    Unhandled,
}

pub fn sbi_handler_panic(trap_frame: &TrapFrame) {
    panic!(
        "Unhandled exception! mstatus: {:x?}, mcause: {:?}, mepc: {:08x?}, insn:{:x},  mtval: {:08x?}, trap frame: {:x?}",
        mstatus::read(),
        mcause::read().cause(),
        mepc::read(),
        unsafe {super::access::get_insn(mepc::read())},
        mtval::read(),
        trap_frame
    );
}
