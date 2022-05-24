extern "C" {
    fn __arch_hart_id() -> usize;
    fn __arch_save_flag() -> usize;
    fn __arch_restore_flag(flag: usize);
    fn __arch_mem_wb(start: usize, size: usize);
    fn __arch_mem_flush(start: usize, size: usize);
    fn __arch_mem_invalid(start: usize, size: usize);
}

#[no_mangle]
pub extern "C" fn hartid() -> usize {
    unsafe { __arch_hart_id() }
}

#[no_mangle]
pub extern "C" fn save_flag() -> usize {
    unsafe { __arch_save_flag() }
}

#[no_mangle]
pub extern "C" fn restore_flag(flag: usize) {
    unsafe { __arch_restore_flag(flag) }
}

#[no_mangle]
pub extern "C" fn mem_wb(start: usize, size: usize) {
    unsafe { __arch_mem_wb(start, size) }
}

#[no_mangle]
pub extern "C" fn mem_flush(start: usize, size: usize) {
    unsafe { __arch_mem_flush(start, size) }
}

#[no_mangle]
pub extern "C" fn mem_invalid(start: usize, size: usize) {
    unsafe { __arch_mem_invalid(start, size) }
}
