#[cfg(target_pointer_width = "64")]
macro_rules! LWU_STR {
    () => {
        "lwu"
    };
}
#[cfg(target_pointer_width = "32")]
macro_rules! LWU_STR {
    () => {
        "lw"
    };
}

#[cfg(target_pointer_width = "64")]
macro_rules! XLEN_MINUS_16 {
    () => {
        "48"
    };
}

#[cfg(target_pointer_width = "32")]
macro_rules! XLEN_MINUS_16 {
    () => {
        "16"
    };
}

pub unsafe fn get_insn(vaddr: usize) -> u32 {
    let mut ans: u32;
    core::arch::asm!(concat!("
                    li      t0, (1 << 17)
                    li      t3, 3
                    and     t2, {vaddr}, 2
                    csrrs   t0, mstatus, t0
                    bnez    t2, 1f
                    ", LWU_STR!(), " t1, 0({vaddr})
                    and t2, t1, 3
                    beq t2, t3, 2f
                    sll t1, t1, ",XLEN_MINUS_16!(),"
                    srl t1, t1, ",XLEN_MINUS_16!(),"
                    j 2f
                    1:
                    lhu t1, 0({vaddr})
                    and t2, t1, 3
                    bne t2, t3, 2f
                    lhu t2, 2({vaddr})
                    sll t2, t2, 16
                    add t1, t1, t2
                    2:
                    csrw    mstatus, t0
                    mv      {ans}, t1
                "), ans = out(reg) ans,  vaddr = in(reg) vaddr);
    ans
}

pub unsafe fn sbi_io_read64(ptr: usize) -> u64 {
    let mut value_l: u32;
    let mut value_h: u32;
    core::arch::asm!(
        "li      t0, (1 << 17)
        csrrs   t0, mstatus, t0
        lw {1}, 0({0})
        lw {2}, 4({0})
        csrw    mstatus, t0
        ",
        in(reg) ptr,
        out(reg) value_l,
        out(reg) value_h,
    );
    value_l as u64 | ((value_h as u64) << 32)
}

pub unsafe fn sbi_io_read32(ptr: usize) -> u32 {
    let mut value: u32;
    core::arch::asm!(
        "li      t0, (1 << 17)
        csrrs   t0, mstatus, t0
        lw {1}, 0({0})
        csrw    mstatus, t0
        ",
        in(reg) ptr,
        out(reg) value,
    );
    value
}

pub unsafe fn sbi_io_read16(ptr: usize) -> u16 {
    let mut value: u16;
    core::arch::asm!(
        "li      t0, (1 << 17)
            csrrs   t0, mstatus, t0
            lh {1}, 0({0})
            csrw    mstatus, t0
            ",
        in(reg) ptr,
        out(reg) value,
    );
    value
}

pub unsafe fn sbi_io_read8(ptr: usize) -> u8 {
    let mut value: u8;
    core::arch::asm!(
        "li      t0, (1 << 17)
            csrrs   t0, mstatus, t0
            lb {1}, 0({0})
            csrw    mstatus, t0
            ",
        in(reg) ptr,
        out(reg) value,
    );
    value
}

pub unsafe fn sbi_io_write64(ptr: usize, value: u64) {
    let value_l = value as u32;
    let value_h = (value >> 32) as u32;
    core::arch::asm!(
        "li      t0, (1 << 17)
        csrrs   t0, mstatus, t0
        sw {1}, 0({0})
        sw {2}, 4({0})
        csrw    mstatus, t0
        ",
        in(reg) ptr,
        in(reg) value_l,
        in(reg) value_h,
    );
}

pub unsafe fn sbi_io_write32(ptr: usize, value: u32) {
    core::arch::asm!(
        "li      t0, (1 << 17)
        csrrs   t0, mstatus, t0
        sw {1}, 0({0})
        csrw    mstatus, t0
        ",
        in(reg) ptr,
        in(reg) value,
    );
}

pub unsafe fn sbi_io_write16(ptr: usize, value: u16) {
    core::arch::asm!(
        "li      t0, (1 << 17)
        csrrs   t0, mstatus, t0
        sh {1}, 0({0})
        csrw    mstatus, t0
        ",
        in(reg) ptr,
        in(reg) value,
    );
}

pub unsafe fn sbi_io_write8(ptr: usize, value: u8) {
    core::arch::asm!(
        "li      t0, (1 << 17)
        csrrs   t0, mstatus, t0
        sb {1}, 0({0})
        csrw    mstatus, t0
        ",
        in(reg) ptr,
        in(reg) value,
    );
}
