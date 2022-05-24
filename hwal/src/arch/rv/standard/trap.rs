use crate::arch::rv::riscv;
use riscv::register::mtvec::{self, TrapMode};
core::arch::global_asm!(include_str!("trap.S"));

pub fn init_trap(mode: TrapMode) {
    extern "C" {
        static _vector_table: usize;
    }
    let m_vector_table = unsafe { &_vector_table } as *const usize as usize;
    unsafe {
        mtvec::write(m_vector_table, mode);
    }
}