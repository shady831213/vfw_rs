use crate::{fill_usize, load_usize};
#[cfg(not(feature = "stack_non_priv"))]
#[macro_use]
mod stack_init {
    macro_rules! load_sp {
        () => {
            load_usize!(sp, sstack_symbol)
        };
    }
}
#[cfg(feature = "stack_non_priv")]
#[macro_use]
mod stack_init {
    macro_rules! load_sp {
        () => {
            concat!(
                load_usize!(sp, estack_symbol),
                "
                ",
                load_usize!(t0, stack_size_symbol),
                "
            csrr t1, mhartid
            addi t1, t1,  1
        1:  add  sp, sp, t0
            addi t1, t1, -1
            bnez t1, 1b
            "
            )
        };
    }
}

#[unsafe(naked)]
#[no_mangle]
#[link_section = ".init"]
unsafe extern "C" fn _start() {
    core::arch::naked_asm!(
        // some platform need reset registers manually
        "
        li  x1, 0
        li  x2, 0
        li  x3, 0
        li  x1, 0
        li  x2, 0
        li  x3, 0
        li  x4, 0
        li  x5, 0
        li  x6, 0
        li  x7, 0
        li  x8, 0
        li  x9, 0
        li  x10,0
        li  x11,0
        li  x12,0
        li  x13,0
        li  x14,0
        li  x15,0
        li  x16,0
        li  x17,0
        li  x18,0
        li  x19,0
        li  x20,0
        li  x21,0
        li  x22,0
        li  x23,0
        li  x24,0
        li  x25,0
        li  x26,0
        li  x27,0
        li  x28,0
        li  x29,0
        li  x30,0
        li  x31,0
        .option push
        .option norelax
        .option nopic
        la t0, {abort}
        csrw mtvec, t0
    ",
    load_usize!(gp, gp_symbol),
    load_sp!(),
    "
    call {move_stack}
    ",
    "
    call {relocation}
    fence.i
    ",
    load_usize!(ra, 1f),
    "
    .option pop
    jr ra
    .align 3
    ",
    fill_usize!(1, "{vfw_start}"),
        abort = sym abort,
        relocation = sym crate::vfw_relocation,
        move_stack          =   sym crate::arch::reuse_stack_for_trap,
        vfw_start = sym crate::vfw_start
    )
}

core::arch::global_asm!(
    "
    .section .init.got
    .align 3
    ",
    fill_usize!(gp_symbol, "__global_pointer$"),
    fill_usize!(sstack_symbol, _sstack),
    fill_usize!(estack_symbol, _estack),
    fill_usize!(stack_size_symbol, _stack_size),
);

#[unsafe(naked)]
#[link_section = ".init.abort"]
unsafe extern "C" fn abort() {
    core::arch::naked_asm!(
        "
        .align 2
        wfi
        j {abort}
        ",
        abort = sym abort
    )
}
