use crate::sys::num_cores;
extern "C" {
    fn __send_ipi(hart_id: usize);
    fn __clear_ipi(hart_id: usize);
    fn __wait_ipi();
}

pub fn wait_ipi() {
    unsafe {
        __wait_ipi();
    }
}

pub fn send_ipi(target: usize) -> Result<(), &'static str> {
    if target >= num_cores() as usize {
        return Err("invalid target!");
    }
    unsafe { __send_ipi(target) };
    Ok(())
}

pub fn clear_ipi(target: usize) -> Result<(), &'static str> {
    if target >= num_cores() as usize {
        return Err("invalid target!");
    }
    unsafe { __clear_ipi(target) };
    Ok(())
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
