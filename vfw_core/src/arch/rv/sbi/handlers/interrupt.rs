use crate::{clear_ipi, hartid};
use riscv::register::{mie, mip};
pub extern "C" fn sbi_timer_handler() {
    unsafe {
        mie::clear_mtimer();
        mip::set_stimer();
    }
}

pub extern "C" fn sbi_soft_handler() {
    unsafe {
        clear_ipi(hartid());
        mip::set_ssoft();
    }
}
