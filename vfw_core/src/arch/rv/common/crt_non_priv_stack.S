    .section .init, "ax"
    .global _start
_start:
    .globl _abs_start
_abs_start:
    .option push
    .option norelax
    la  gp, __global_pointer$
    .option pop
    la  sp, _estack
    lui t0, %hi(_stack_size)
    addi t0, t0, %lo(_stack_size)
    csrr t1, mhartid
    addi t1, t1,  1
1:  add  sp, sp, t0
    addi t1, t1, -1
    bnez t1, 1b
    j vfw_start

_start_abort:
    wfi
    j   _start_abort