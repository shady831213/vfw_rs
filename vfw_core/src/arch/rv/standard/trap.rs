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
        "j {default}", // supervisor software
        "j {default}", // reserved
        "j {default} ",  // machine    software
        "j {default}", // reserved
        "j {default}", // supervisor timer
        "j {default}", // reserved
        "j {default}",  // machine    timer
        "j {default}", // reserved
        "j {default}", // supervisor external
        "j {default}", // reserved
        "j {default}", // machine    external
        ".option pop",
        default = sym fast_trap::trap_entry,
        options(noreturn)
    )
}
