use crate::rsrt::*;
pub fn rv_wait_ipi() {
    use riscv::asm::wfi;
    use riscv::register::mie;
    unsafe {
        let sie = mie::read().msoft();
        mie::set_msoft();
        wfi();
        if !sie {
            mie::clear_msoft();
        }
    }
}

pub fn rv_hart_id() -> usize {
    riscv::register::mhartid::read()
}

use riscv::register::mstatus;

pub fn rv_save_flag() -> usize {
    unsafe {
        let flag = mstatus::read().mie() as usize;
        mstatus::clear_mie();
        flag
    }
}

pub fn rv_restore_flag(flag: usize) {
    unsafe {
        if flag != 0 {
            mstatus::set_mie();
        }
    }
}

pub fn run_task(task: &Task) {
    unsafe {
        asm!("mv a0, $1
        mv a1, $2
        mv a2, $3
        mv a3, $4
        mv a4, $5
        mv a5, $6
        mv a6, $7
        mv a7, $8
        jalr ra,$0",
                in(ra) task.entry, in(a0) task.args[0], in(a1) task.args[1], in(a2) task.args[2], in(a3) task.args[3], in(a4) task.args[4], in(a5) task.args[5], in(a6) task.args[6], in(a7) task.args[7]);
    }
}
