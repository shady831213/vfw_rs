use crate::num_cores;

#[linkage = "weak"]
#[no_mangle]
pub extern "C" fn mem_wb(_start: usize, _size: usize) {}

#[linkage = "weak"]
#[no_mangle]
pub extern "C" fn mem_flush(_start: usize, _size: usize) {}

#[linkage = "weak"]
#[no_mangle]
pub extern "C" fn mem_invalid(_start: usize, _size: usize) {}

pub fn wait_ipi() {
    extern "C" {
        fn __wait_ipi();
    }
    unsafe {
        __wait_ipi();
    }
}

pub fn send_ipi(target: usize) {
    extern "C" {
        fn __send_ipi(hart_id: usize);
    }
    if target >= num_cores() as usize {
        panic!("send_ipi invalid target {}", target);
    }
    unsafe { __send_ipi(target) };
}

pub fn clear_ipi(target: usize) {
    extern "C" {
        fn __clear_ipi(hart_id: usize);
    }
    if target >= num_cores() as usize {
        panic!("clear_ipi invalid target {}", target);
    }
    unsafe { __clear_ipi(target) };
}

pub fn send_ipis(targets: usize) {
    for i in 0..num_cores() as usize {
        if targets & (1 << i) != 0 {
            send_ipi(i);
        }
    }
}

pub fn send_others_ipis(hartid: usize, targets: usize) {
    send_ipis(targets & !(1 << hartid))
}
