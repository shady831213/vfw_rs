#[cfg(not(feature = "stack_non_priv"))]
#[macro_use]
mod stack_init {
    macro_rules! laod_sp {
        () => {
            "
        la  sp, _sstack
        "
        };
    }
}
#[cfg(feature = "stack_non_priv")]
#[macro_use]
mod stack_init {
    macro_rules! laod_sp {
        () => {
            "
            la  sp, _estack
            lui t0, %hi(_stack_size)
            addi t0, t0, %lo(_stack_size)
            lui t1, %hi(_provide_base)
            addi t1, t1, %lo(_provide_base)
            sub t0, t0, t1
            csrr t1, mhartid
            addi t1, t1,  1
        1:  add  sp, sp, t0
            addi t1, t1, -1
            bnez t1, 1b
            "
        };
    }
}

#[naked]
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
        la t0, {abort}
        csrw mtvec, t0
        la  gp, __global_pointer$
        .option pop
    ",
    laod_sp!(),
    "call {move_stack}
    j {vfw_start}
    ",
        abort = sym abort,
        move_stack          =   sym crate::arch::reuse_stack_for_trap,
        vfw_start = sym crate::vfw_start
    )
}

#[naked]
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
