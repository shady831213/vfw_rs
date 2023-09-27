pub use super::super::common::trap::*;
use riscv::register::mtvec::{self, TrapMode};
// core::arch::global_asm!(include_str!("trap.S"));

pub fn init_trap(mode: TrapMode) {
    // extern "C" {
    //     static _vector_table: usize;
    // }
    // let m_vector_table = unsafe { &_vector_table } as *const usize as usize;
    unsafe {
        mtvec::write(trap_vec as usize, mode);
    }
}

#[naked]
unsafe extern "C" fn trap_vec() {
    core::arch::asm!(
        ".align 2",
        ".option push",
        ".option norvc",
        "j {default}", // exception
        "j _start_trap", // supervisor software
        "j _start_trap", // reserved
        "j _start_trap ",  // machine    software
        "j _start_trap", // reserved
        "j _start_trap", // supervisor timer
        "j _start_trap", // reserved
        "j _start_trap",  // machine    timer
        "j _start_trap", // reserved
        "j _start_trap", // supervisor external
        "j _start_trap", // reserved
        "j _start_trap", // machine    external
        ".option pop",
        default = sym fast_trap::trap_entry,
        options(noreturn)
    )
}
