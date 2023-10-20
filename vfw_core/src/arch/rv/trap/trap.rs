use crate::*;
use paste::paste;
use riscv::register::{
    mcause::{self, Exception as E, Trap as T},
    mstatus,
};

macro_rules! exchange {
    () => {
        exchange!(sp)
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

    macro_rules! push_stack {
        ($size:expr) => {
            concat!("addi sp, sp, ", "-4*", $size)
        };
    }

    macro_rules! pop_stack {
        ($size:expr) => {
            concat!("addi sp, sp, ", "4*", $size)
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

    macro_rules! push_stack {
        ($size:expr) => {
            concat!("addi sp, sp, ", "-8*", $size)
        };
    }

    macro_rules! pop_stack {
        ($size:expr) => {
            concat!("addi sp, sp, ", "8*", $size)
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

macro_rules! on_vfw_stack {
    ($name:ident, $entry: ident) => {
        paste! {
            #[naked]
            unsafe extern "C" fn [<$name _on_vfw_stack>]() {
                core::arch::asm!(
                    exchange!(),
                    push_stack!(1),
                    save!(ra => sp[0]),
                    "call {handler}",
                    load!(sp[0] => ra),
                    pop_stack!(1),
                    exchange!(),
                    "ret",
                    handler    = sym $entry,
                    options(noreturn),
                )
            }
        }
    };
}

//only for machine level vfw run-time
//if need switch to other context, such as SBI, Stack.load_as_stack can be used to set other context and fast_handler
//for sbi, all machine level run time is in the trap scope, thus, all stack is available for fast_trap
//so vfw_fast_handler only handle machine level app
#[inline(always)]
pub(crate) extern "C" fn vfw_handler(ctx: &mut FlowContext) {
    //FIXME:just for compile
}

// #[naked]
pub(crate) fn trap_entry() {}

#[repr(C)]
#[allow(missing_docs)]
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
    /// 零初始化。
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
