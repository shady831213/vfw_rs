use crate::cpu_ctx;
use riscv::register::{mhartid, mstatus};

pub(crate) fn hartid() -> usize {
    mhartid::read()
}

pub(crate) fn save_flag() -> usize {
    unsafe {
        let flag = mstatus::read().mie() as usize;
        mstatus::clear_mie();
        flag
    }
}

pub(crate) fn restore_flag(flag: usize) {
    unsafe {
        if flag != 0 {
            mstatus::set_mie();
        }
    }
}

pub(crate) fn init_fast_trap() {
    unsafe {
        core::arch::asm!("
        mv {gp}, gp
        ", 
        gp = out(reg) cpu_ctx(hartid()).trap.gp,
        );
    }
}
