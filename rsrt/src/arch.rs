use crate::hw_thread::Task;
union ArchRunTask {
    handler: fn(task: &Task),
    reserved: usize,
}

#[repr(C)]
struct ArchHandlers {
    run_task: ArchRunTask,
}

extern "C" {
    fn __arch_hart_id() -> usize;
    fn __arch_save_flag() -> usize;
    fn __arch_restore_flag(flag: usize);
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

pub fn set_arch_task_run(f: fn(task: &Task)) {
    unsafe { ARCH.run_task.handler = f }
}

pub(crate) fn run_task() -> Result<fn(task: &Task), &'static str> {
    unsafe {
        if ARCH.run_task.reserved == 0 {
            Err("not support hw_thread! run_task handler is not set!")
        } else {
            Ok(ARCH.run_task.handler)
        }
    }
}

static mut ARCH: ArchHandlers = ArchHandlers {
    run_task: ArchRunTask { reserved: 0 },
};
