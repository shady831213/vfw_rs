use crate::num_cores;
extern "C" {
    fn __save_flag() -> usize;
    fn __restore_flag(flag: usize);
    fn __mem_wb(start: usize, size: usize);
    fn __mem_flush(start: usize, size: usize);
    fn __mem_invalid(start: usize, size: usize);
    fn __send_ipi(hart_id: usize);
    fn __clear_ipi(hart_id: usize);
    fn __wait_ipi();
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

pub fn wait_ipi() {
    unsafe {
        __wait_ipi();
    }
}

pub fn send_ipi(target: usize) {
    if target >= num_cores() as usize {
        return;
    }
    unsafe { __send_ipi(target) };
}

pub fn clear_ipi(target: usize) {
    if target >= num_cores() as usize {
        return;
    }
    unsafe { __clear_ipi(target) };
}

pub fn send_ipis(targets: usize) {
    for i in 0..num_cores() as usize {
        if targets & (1 << i) != 0 {
            unsafe { __send_ipi(i) };
        }
    }
}

pub fn send_others_ipis(hartid: usize, targets: usize) {
    send_ipis(targets & !(1 << hartid))
}
