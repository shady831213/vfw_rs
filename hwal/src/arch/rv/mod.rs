pub extern crate riscv;
pub mod rtapi;
pub mod sbi;
pub mod standard;
pub mod trap;
pub mod pmp;
core::arch::global_asm!(include_str!("crt.S"));
#[macro_export]
macro_rules! read_csr {
    ($csr_number:expr) => {
        {
            unsafe {
                let r: usize;
                core::arch::asm!("csrrs {0}, {1}, x0", out(reg) r, const $csr_number, options(pure, nomem, nostack));
                r
            }
        }
    }
}

#[macro_export]
macro_rules! write_csr {
    ($csr_number:expr, $value:expr) => {
        unsafe { core::arch::asm!("csrrw x0, {1}, {0}",  in(reg) $value, const $csr_number, options(nomem, nostack)) }
    }
}

#[macro_export]
macro_rules! set_csr {
    ($csr_number:expr, $value:expr) => {
        unsafe { core::arch::asm!("csrrs x0, {1}, {0}",  in(reg) $value, const $csr_number, options(nomem, nostack)) }
    }
}

#[macro_export]
macro_rules! clr_csr {
    ($csr_number:expr, $value:expr) => {
        unsafe { core::arch::asm!("csrrc x0, {1}, {0}",  in(reg) $value, const $csr_number, options(nomem, nostack)) }
    }
}

#[no_mangle]
pub fn wait_mcycle(cnt: usize) {
    let cur = riscv::register::mcycle::read();
    while riscv::register::mcycle::read().wrapping_sub(cur) < cnt {}
}

#[no_mangle]
pub fn mcycle() -> usize {
    riscv::register::mcycle::read()
}
