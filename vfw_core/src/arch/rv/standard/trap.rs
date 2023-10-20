use riscv::register::mtvec::{self, TrapMode};
pub fn init_trap(mode: TrapMode) {
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
        default = sym crate::arch::trap_entry,
        options(noreturn)
    )
}
