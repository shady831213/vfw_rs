use super::exception::*;
use super::interrupt::*;
use crate::TrapContext;
use core::alloc::Layout;
use riscv::register::{
    mcause::{self, Trap as T},
    mepc, mstatus, mtval,
};
macro_rules! exchange {
    () => {
        exchange!(tp)
    };

    ($reg:ident) => {
        concat!("csrrw ", stringify!($reg), ", mscratch, ", stringify!($reg))
    };
}

#[cfg(target_pointer_width = "32")]
#[macro_use]
mod arch {
    macro_rules! save {
        ($reg:ident => $ptr:ident[$pos:expr]) => {
            concat!(
                "sw ",
                stringify!($reg),
                ", 4*",
                $pos,
                '(',
                stringify!($ptr),
                ')'
            )
        };
    }

    macro_rules! load {
        ($ptr:ident[$pos:expr] => $reg:ident) => {
            concat!(
                "lw ",
                stringify!($reg),
                ", 4*",
                $pos,
                '(',
                stringify!($ptr),
                ')'
            )
        };
    }
}
#[cfg(target_pointer_width = "64")]
#[macro_use]
mod arch {
    macro_rules! save {
        ($reg:ident => $ptr:ident[$pos:expr]) => {
            concat!(
                "sd ",
                stringify!($reg),
                ", 8*",
                $pos,
                '(',
                stringify!($ptr),
                ')'
            )
        };
    }

    macro_rules! load {
        ($ptr:ident[$pos:expr] => $reg:ident) => {
            concat!(
                "ld ",
                stringify!($reg),
                ", 8*",
                $pos,
                '(',
                stringify!($ptr),
                ')'
            )
        };
    }
}

#[inline(always)]
pub extern "C" fn trap_handler(ctx: &mut FlowContext) {
    match mcause::read().cause() {
        T::Exception(_) => exception_handler(ctx),
        T::Interrupt(i) => interrupt_handler(ctx, i),
    }
}

pub fn default_trap_handler(ctx: &mut FlowContext) {
    panic!(
        "[core{}] Unhandled exception!  mcause: {:?}, mepc: {:#x}, mtval: {:#x}, mstatus: {:#x?}, ctx:{:#x?}",
        crate::hartid(),
        mcause::read().cause(),
        mepc::read(),
        mtval::read(),
        mstatus::read(),
        ctx,
    );
}

pub fn dummy_trap_handler() {}

//tp in machine mode usually not used
//we don't swap sp by default, if sp should be swapped, handle it in handler
#[naked]
pub unsafe extern "C" fn trap_entry() {
    core::arch::naked_asm!(
        ".align 2",
        exchange!(),
        //save a0 to scrach
        save!(a0 => tp[2]),
        load!(tp[0] => a0),
        save!(ra => a0[0]),
        save!(t0 => a0[1]),
        save!(t1 => a0[2]),
        save!(t2 => a0[3]),
        save!(t3 => a0[4]),
        save!(t4 => a0[5]),
        save!(t5 => a0[6]),
        save!(t6 => a0[7]),
        save!(a1 => a0[9]),
        save!(a2 => a0[10]),
        save!(a3 => a0[11]),
        save!(a4 => a0[12]),
        save!(a5 => a0[13]),
        save!(a6 => a0[14]),
        save!(a7 => a0[15]),
        save!(s0  => a0[16]),
        save!(s1  => a0[17]),
        save!(s2  => a0[18]),
        save!(s3  => a0[19]),
        save!(s4  => a0[20]),
        save!(s5  => a0[21]),
        save!(s6  => a0[22]),
        save!(s7  => a0[23]),
        save!(s8  => a0[24]),
        save!(s9  => a0[25]),
        save!(s10 => a0[26]),
        save!(s11 => a0[27]),
        save!(gp => a0[28]),
        save!(sp => a0[30]),
        // save original a0
        load!(tp[2] => t0),
        save!(t0 => a0[8]),
        // save original tp
        "csrr t0, mscratch",
        save!(t0 => a0[29]),
        // save mepc
        "csrr t0, mepc",
        save!(t0 => a0[31]),
        //handler
        load!(tp[1] => ra),
        "jalr ra",
        load!(tp[0] => a1),
        // resotre new tp or original
        load!(a1[29] => t0),
        "csrw mscratch, t0",
        // resotre new pc
        load!(a1[31] => t0),
        "csrw mepc, t0",
        //restore others
        load!(a1[0] => ra),
        load!(a1[1] => t0),
        load!(a1[2] => t1),
        load!(a1[3] => t2),
        load!(a1[4] => t3),
        load!(a1[5] => t4),
        load!(a1[6] => t5),
        load!(a1[7] => t6),
        load!(a1[8] => a0),
        load!(a1[10] => a2),
        load!(a1[11] => a3),
        load!(a1[12] => a4),
        load!(a1[13] => a5),
        load!(a1[14] => a6),
        load!(a1[15] => a7),
        load!(a1[16] => s0 ),
        load!(a1[17] => s1 ),
        load!(a1[18] => s2 ),
        load!(a1[19] => s3 ),
        load!(a1[20] => s4 ),
        load!(a1[21] => s5 ),
        load!(a1[22] => s6 ),
        load!(a1[23] => s7 ),
        load!(a1[24] => s8 ),
        load!(a1[25] => s9 ),
        load!(a1[26] => s10),
        load!(a1[27] => s11),
        load!(a1[28] => gp),
        load!(a1[30] => sp),
        load!(a1[9] => a1),
        exchange!(),
        "   mret"
    )
}

#[derive(Debug)]
#[repr(C)]
pub struct FlowContext {
    pub ra: usize,      // 0..
    pub t: [usize; 7],  // 1..
    pub a: [usize; 8],  // 8..
    pub s: [usize; 12], // 16..
    pub gp: usize,      // 28..
    pub tp: usize,      // 29..
    pub sp: usize,      // 30..
    pub pc: usize,      // 31..
}

impl FlowContext {
    pub const ZERO: Self = Self {
        ra: 0,
        t: [0; 7],
        a: [0; 8],
        s: [0; 12],
        gp: 0,
        tp: 0,
        sp: 0,
        pc: 0,
    };
}

#[naked]
#[link_section = ".init.trap"]
pub unsafe extern "C" fn reuse_stack_for_trap() {
    const LAYOUT: Layout = Layout::new::<TrapContext>();
    core::arch::naked_asm!(
        "   addi sp, sp, {size}
            andi sp, sp, {mask}
            ret
        ",
        size = const -(LAYOUT.size() as isize),
        mask = const !(LAYOUT.align() as isize - 1)
    )
}
