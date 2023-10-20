pub fn io_write32_with_err(addr: *mut u32, data: u32) -> Result<(), u32> {
    let mut err: u32 = 0;
    unsafe {
        core::arch::asm!(
            "sw {1}, 0({0})",
            in(reg) addr,
            in(reg) data,
            inout("t0") err,
        )
    }
    if err == 0 {
        Ok(())
    } else {
        Err(err)
    }
}

#[no_mangle]
extern "C" fn c_io_write32_with_err(addr: usize, data: u32, err: &mut bool) {
    let r = io_write32_with_err(addr as *mut u32, data);
    if r.is_err() {
        *err = true
    } else {
        *err = false
    }
}

pub fn io_read32_with_err(addr: *mut u32) -> Result<u32, u32> {
    let mut err: u32 = 0;
    let mut data: u32;
    unsafe {
        core::arch::asm!(
            "lw {1}, 0({0})",
            in(reg) addr,
            out(reg) data,
            inout("t0") err,
        )
    }
    if err == 0 {
        Ok(data)
    } else {
        Err(err)
    }
}

#[no_mangle]
extern "C" fn c_io_read32_with_err(addr: usize, err: &mut bool) -> u32 {
    let r = io_read32_with_err(addr as *mut u32);
    if let Ok(data) = r {
        *err = false;
        data
    } else {
        *err = true;
        0
    }
}

pub fn check_write32_with_err(addr: *mut u32, pattern: u32, write_ignore: bool) -> Result<(), ()> {
    let sd = io_write32_with_err(addr, pattern);
    if write_ignore {
        if !sd.is_ok() {
            crate::println!(
                "WRITE FAIL {:x}! expect write being ignored, but get a excepiton!",
                addr as usize
            );
            return Err(());
        }
        Ok(())
    } else {
        if !sd.is_err() {
            crate::println!(
                "WRITE FAIL {:x}! expect an exception, but write is ignored!",
                addr as usize
            );
            return Err(());
        }
        Ok(())
    }
}

pub fn check_read32_with_err(addr: *mut u32, read_ignore: bool) -> Result<(), ()> {
    let ld = io_read32_with_err(addr);
    if read_ignore {
        if !ld.is_ok() {
            crate::println!(
                "READ FAIL {:x}! expect read being ignored, but get a excepiton!",
                addr as usize
            );
            return Err(());
        }
        Ok(())
    } else {
        if !ld.is_err() {
            crate::println!(
                "READ FAIL {:x}! expect an exception, but read is ignored!",
                addr as usize
            );
            return Err(());
        }
        Ok(())
    }
}

pub unsafe fn io_access_err_expt_handler(ctx: &mut crate::arch::FlowContext) {
    use riscv::register::mepc;
    use vfw_primitives::io_read8;
    let is_c = (io_read8!(mepc::read()) & 0x3) != 0x3;
    ctx.t[0] = 0xdeadbeaf;
    if is_c {
        mepc::write(mepc::read().wrapping_add(2))
    } else {
        mepc::write(mepc::read().wrapping_add(4))
    }
}
