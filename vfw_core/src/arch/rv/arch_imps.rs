use crate::{cpu_ctx, Task};
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

pub(crate) fn boot(task: &Task) {
    unsafe {
        core::arch::asm!(
        "jalr ra,{0}",in(reg) task.entry,
        in("a0") task.args[0],
        in("a1") task.args[1],
        in("a2") task.args[2],
        in("a3") task.args[3],
        in("a4") task.args[4],
        in("a5") task.args[5],
        in("a6") task.args[6],
        in("a7") task.args[7],
        clobber_abi("C"),);
    }
}

#[inline]
pub(crate) fn exchange_scratch(mut val: usize) -> usize {
    unsafe { core::arch::asm!("csrrw {0}, mscratch, {0}", inlateout(reg) val) };
    val
}
