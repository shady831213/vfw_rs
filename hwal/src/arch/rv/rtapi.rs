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
        llvm_asm!("mv a0, $1
        mv a1, $2
        mv a2, $3
        mv a3, $4
        mv a4, $5
        mv a5, $6
        mv a6, $7
        mv a7, $8
        jalr ra,$0"
                :
                :"r"(task.entry), "r"(task.args[0]), "r"(task.args[1]), "r"(task.args[2]), "r"(task.args[3]), "r"(task.args[4]), "r"(task.args[5]), "r"(task.args[6]), "r"(task.args[7])
                :"ra", "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7");
    }
}
