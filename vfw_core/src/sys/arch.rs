extern "C" {
    fn __hart_id() -> usize;
    fn __save_flag() -> usize;
    fn __restore_flag(flag: usize);
    fn __mem_wb(start: usize, size: usize);
    fn __mem_flush(start: usize, size: usize);
    fn __mem_invalid(start: usize, size: usize);
}

#[no_mangle]
pub extern "C" fn hartid() -> usize {
    unsafe { __hart_id() }
}

#[no_mangle]
pub extern "C" fn save_flag() -> usize {
    unsafe { __save_flag() }
}

#[no_mangle]
pub extern "C" fn restore_flag(flag: usize) {
    unsafe { __restore_flag(flag) }
}

#[no_mangle]
pub extern "C" fn mem_wb(start: usize, size: usize) {
    unsafe { __mem_wb(start, size) }
}

#[no_mangle]
pub extern "C" fn mem_flush(start: usize, size: usize) {
    unsafe { __mem_flush(start, size) }
}

#[no_mangle]
pub extern "C" fn mem_invalid(start: usize, size: usize) {
    unsafe { __mem_invalid(start, size) }
}

#[no_mangle]
pub extern "C" fn num_cores() -> usize {
    extern "C" {
        static _num_cores: u8;
    }
    (unsafe { &_num_cores }) as *const u8 as usize
}
