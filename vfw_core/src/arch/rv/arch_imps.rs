use crate::Task;
use crate::{fill_usize, load_usize};
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

#[link_section = ".init"]
pub(crate) fn reloc_got() {
    unsafe {
        core::arch::asm!(
            "
            .option push
            .option norelax
            .option nopic
            ",
            load_usize!(t0, sgot_symbol),
            load_usize!(t1, sgot_load_symbol),
            "
            beq t0, t1, 2f
            ",
            load_usize!(t2, egot_symbol),
            "
            beq t0, t2, 2f
            1:
            lw t3, 0(t1)
            sw t3, 0(t0)
            addi t1, t1, 4
            addi t0, t0, 4
            bltu t0, t2, 1b
            2:
            .option pop
        ",
            clobber_abi("C"),
        )
    }
}

#[allow(dead_code)]
#[link_section = ".init"]
pub(crate) fn init_num_cores() -> usize {
    unsafe {
        let r: u32;
        core::arch::asm!(
            "
            .option push
            .option norelax
            .option nopic
            ",
            load_usize!(t0, num_cores_symbol),
            "
            .option pop
            ",
            out("t0") r,
            clobber_abi("C"),
        );
        r as _
    }
}

core::arch::global_asm!(
    "
    .section .init.got
    .align 3
    ",
    fill_usize!(sgot_symbol, _sgot),
    fill_usize!(egot_symbol, _egot),
    fill_usize!(sgot_load_symbol, _sgot_load),
    fill_usize!(num_cores_symbol, _num_cores),
);

#[inline]
pub(crate) fn exchange_scratch(mut val: usize) -> usize {
    unsafe { core::arch::asm!("csrrw {0}, mscratch, {0}", inlateout(reg) val) };
    val
}
