.global _start_trap
_start_trap:
    csrrw   sp, mscratch, sp
    bnez    sp, 1f
    /* from M level, load sp */
    csrrw   sp, mscratch, zero
1:
    addi    sp, sp, -32 * REGBYTES
    STORE   x1, 1
    STORE   x2, 2
    STORE   x3, 3
    STORE   x4, 4
    STORE   x5, 5
    STORE   x6, 6
    STORE   x7, 7
    STORE   x8, 8
    STORE   x9, 9
    STORE   x10, 10
    STORE   x11, 11
    STORE   x12, 12
    STORE   x13, 13
    STORE   x14, 14
    STORE   x15, 15
    STORE   x16, 16
    STORE   x17, 17
    STORE   x18, 18
    STORE   x19, 19
    STORE   x20, 20
    STORE   x21, 21
    STORE   x22, 22
    STORE   x23, 23
    STORE   x24, 24
    STORE   x25, 25
    STORE   x26, 26
    STORE   x27, 27
    STORE   x28, 28
    STORE   x29, 29
    STORE   x30, 30
    STORE   x31, 31
    mv      a0, sp
    call    start_trap_rust
    LOAD   x1, 1
    LOAD   x2, 2
    LOAD   x3, 3
    LOAD   x4, 4
    LOAD   x5, 5
    LOAD   x6, 6
    LOAD   x7, 7
    LOAD   x8, 8
    LOAD   x9, 9
    LOAD   x10, 10
    LOAD   x11, 11
    LOAD   x12, 12
    LOAD   x13, 13
    LOAD   x14, 14
    LOAD   x15, 15
    LOAD   x16, 16
    LOAD   x17, 17
    LOAD   x18, 18
    LOAD   x19, 19
    LOAD   x20, 20
    LOAD   x21, 21
    LOAD   x22, 22
    LOAD   x23, 23
    LOAD   x24, 24
    LOAD   x25, 25
    LOAD   x26, 26
    LOAD   x27, 27
    LOAD   x28, 28
    LOAD   x29, 29
    LOAD   x30, 30
    LOAD   x31, 31
    addi    sp, sp, 32 * REGBYTES
    csrrw   sp, mscratch, sp
    bnez    sp, 1f
    /* from M level, load sp */
    csrrw   sp, mscratch, zero
1:
    mret