#[derive(Copy, Clone)]
#[repr(C)]
pub struct Task {
    pub entry: usize,
    pub args: [usize; 8],
    pub task_id: TaskId,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct TaskId(u32);
impl TaskId {
    pub const fn new(hart_id: u16, task_id: u16) -> Self {
        TaskId((hart_id as u32) | ((task_id as u32) << 16))
    }
    pub const fn from_u32(raw: u32) -> Self {
        TaskId(raw)
    }

    pub fn task_id(&self) -> u16 {
        (self.0 >> 16) as u16
    }
    pub fn hart_id(&self) -> u16 {
        self.0 as u16
    }
    pub fn raw(&self) -> u32 {
        self.0
    }
}

#[cfg(any(
    feature = "max_cores_128",
    feature = "max_cores_64",
    feature = "max_cores_32",
    feature = "max_cores_16",
    feature = "max_cores_8",
    feature = "max_cores_4",
    feature = "max_cores_2"
))]
mod hw_thread_imp {
    use super::*;
    use crate::arch;
    use crate::{clear_ipi, cpu_ctx, hartid, num_cores, send_ipi, wait_ipi};
    use core::sync::atomic::{AtomicU16, Ordering};
    pub(crate) fn thread_loop() {
        loop {
            match unsafe { &mut cpu_ctx(hartid()).hsm.local().start() } {
                Ok(task) => unsafe {
                    cpu_ctx(hartid()).current = task.task_id;
                    arch::boot(&task);
                    cpu_ctx(hartid()).hsm.local().stop();
                    let hart_target = task.task_id.hart_id() as usize;
                    send_ipi(hart_target);
                },
                _ => {
                    wait_ipi();
                    clear_ipi(hartid());
                }
            }
        }
    }
    //should be moved into arch
    #[inline]
    fn __try_fork_on(
        hart_target: usize,
        task_id: u16,
        entry: usize,
        arg_len: usize,
        args: *const usize,
    ) -> Option<TaskId> {
        if hart_target >= num_cores() {
            panic!("Invalid fork target id {}!", hart_target);
        }
        let mut task = Task {
            entry,
            args: [0; 8],
            task_id: TaskId::new(hartid() as u16, task_id),
        };
        for i in 0..arg_len {
            unsafe {
                task.args[i] =
                    *((args as usize + i * core::mem::size_of::<usize>()) as *const usize)
            };
        }
        let ret = cpu_ctx(hart_target).hsm.remote().start(task);
        if ret && hart_target != hartid() {
            send_ipi(hart_target);
        }
        if ret {
            Some(TaskId::new(hart_target as u16, task_id))
        } else {
            None
        }
    }

    //should be moved into arch
    #[inline]
    fn _join(id: TaskId) {
        #[inline(always)]
        fn finished(issued: u16, retired: u16) -> bool {
            if (issued >> 15) != (retired >> 15) {
                retired <= issued
            } else {
                retired >= issued
            }
        }
        let hart_target = id.hart_id() as usize;
        if hart_target >= num_cores() {
            panic!("Invalid join target id {}!", hart_target);
        }
        let cpu = cpu_ctx(hart_target);
        loop {
            wait_ipi();
            clear_ipi(hartid());
            if cpu.hsm.remote().get_status().expect("Invalid State!")
                == crate::hsm::HsmState::Stopped
                && finished(id.task_id(), cpu.current.task_id())
            {
                break;
            }
        }
    }

    #[link_section = ".synced.bss"]
    static HW_TIDS: AtomicU16 = AtomicU16::new(0);

    #[inline]
    pub(crate) fn get_task_id() -> u16 {
        HW_TIDS.fetch_add(1, Ordering::SeqCst)
    }

    #[no_mangle]
    extern "C" fn c_fork(entry: usize, args_len: usize, args: *const usize) -> u32 {
        let task_id = get_task_id();
        loop {
            for i in 1..crate::num_cores() {
                if let Some(id) = __try_fork_on(i, task_id, entry, args_len, args) {
                    return id.raw();
                }
            }
            core::hint::spin_loop();
        }
    }

    #[no_mangle]
    extern "C" fn c_fork_on(
        target_id: usize,
        entry: usize,
        args_len: usize,
        args: *const usize,
    ) -> u32 {
        let task_id = get_task_id();
        loop {
            if let Some(id) = __try_fork_on(target_id, task_id, entry, args_len, args) {
                break id.raw();
            }
            core::hint::spin_loop();
        }
    }

    #[no_mangle]
    extern "C" fn c_try_fork(entry: usize, args_len: usize, args: *const usize) -> u32 {
        let task_id = get_task_id();
        for i in 1..crate::num_cores() {
            if let Some(id) = __try_fork_on(i, task_id, entry, args_len, args) {
                return id.raw();
            }
        }
        -1i32 as u32
    }

    #[no_mangle]
    extern "C" fn c_try_fork_on(
        target_id: usize,
        entry: usize,
        args_len: usize,
        args: *const usize,
    ) -> u32 {
        if let Some(id) = __try_fork_on(target_id, get_task_id(), entry, args_len, args) {
            return id.raw();
        }
        -1i32 as u32
    }

    #[inline]
    pub fn _try_fork_on(target_id: usize, entry: usize, args: &[usize]) -> Option<u32> {
        __try_fork_on(target_id, get_task_id(), entry, args.len(), args.as_ptr()).map(|id| id.raw())
    }

    #[inline]
    pub fn _fork(entry: usize, args: &[usize]) -> u32 {
        let task_id = get_task_id();
        loop {
            for i in 1..crate::num_cores() {
                if let Some(id) = __try_fork_on(i, task_id, entry, args.len(), args.as_ptr()) {
                    return id.raw();
                }
            }
            core::hint::spin_loop();
        }
    }

    #[inline]
    pub fn _fork_on(target_id: usize, entry: usize, args: &[usize]) -> u32 {
        let task_id = get_task_id();
        loop {
            if let Some(id) = __try_fork_on(target_id, task_id, entry, args.len(), args.as_ptr()) {
                break id.raw();
            }
            core::hint::spin_loop();
        }
    }

    #[inline]
    pub fn _try_fork(entry: usize, args: &[usize]) -> Option<u32> {
        let task_id = get_task_id();
        for i in 1..crate::num_cores() {
            if let Some(id) = __try_fork_on(i, task_id, entry, args.len(), args.as_ptr()) {
                return Some(id.raw());
            }
        }
        None
    }

    #[no_mangle]
    pub extern "C" fn join(task_id: u32) {
        _join(TaskId::from_u32(task_id as u32))
    }
}

#[cfg(not(any(
    feature = "max_cores_128",
    feature = "max_cores_64",
    feature = "max_cores_32",
    feature = "max_cores_16",
    feature = "max_cores_8",
    feature = "max_cores_4",
    feature = "max_cores_2"
)))]
mod hw_thread_stub {
    pub(crate) fn thread_loop() {}
    pub(crate) fn get_task_id() -> u16 {
        0
    }

    #[no_mangle]
    extern "C" fn c_fork(_entry: usize, _args_len: usize, _args: *const usize) -> usize {
        panic!("hw_thread not support!")
    }
    #[no_mangle]
    extern "C" fn c_fork_on(
        _target_id: usize,
        _entry: usize,
        _args_len: usize,
        _args: *const usize,
    ) -> u32 {
        panic!("hw_thread not support!")
    }
    #[no_mangle]
    extern "C" fn c_try_fork(_entry: usize, _args_len: usize, _args: *const usize) -> u32 {
        panic!("hw_thread not support!")
    }
    #[no_mangle]
    extern "C" fn c_try_fork_on(
        _target_id: usize,
        _entry: usize,
        _args_len: usize,
        _args: *const usize,
    ) -> u32 {
        panic!("hw_thread not support!")
    }
    pub fn _fork(_entry: usize, _args: &[usize]) -> u32 {
        panic!("hw_thread not support!")
    }
    pub fn _fork_on(_target_id: usize, _entry: usize, _args: &[usize]) -> u32 {
        panic!("hw_thread not support!")
    }
    pub fn _try_fork(_entry: usize, _args: &[usize]) -> Option<u32> {
        panic!("hw_thread not support!")
    }

    pub fn _try_fork_on(_target_id: usize, _entry: usize, _args: &[usize]) -> Option<u32> {
        panic!("hw_thread not support!")
    }
    #[no_mangle]
    pub extern "C" fn join(_task_id: u32) {
        panic!("hw_thread not support!")
    }
}

#[cfg(any(
    feature = "max_cores_128",
    feature = "max_cores_64",
    feature = "max_cores_32",
    feature = "max_cores_16",
    feature = "max_cores_8",
    feature = "max_cores_4",
    feature = "max_cores_2"
))]
pub use hw_thread_imp::*;
#[cfg(not(any(
    feature = "max_cores_128",
    feature = "max_cores_64",
    feature = "max_cores_32",
    feature = "max_cores_16",
    feature = "max_cores_8",
    feature = "max_cores_4",
    feature = "max_cores_2"
)))]
pub use hw_thread_stub::*;

#[macro_export]
macro_rules! fork {
($entry:ident) => {{
    let args:[usize;0] = [0;0];
    crate::_fork($entry as usize, &args)
}};
($entry:ident, $($arg:expr),*) => {{
    let args = [$($arg as usize,)*];
    crate::_fork($entry as usize, &args)
}};
}

#[macro_export]
macro_rules! try_fork {
($entry:ident) => {{
    let args:[usize;0] = [0;0];
    crate::_try_fork($entry as usize, &args)
}};
($entry:ident, $($arg:expr),*) => {{
    let args = [$($arg as usize,)*];
    crate::_try_fork($entry as usize, &args)
}};
}

#[macro_export]
macro_rules! fork_on {
($target:expr, $entry:ident) => {{
    let args:[usize;0] = [0;0];
    crate::_fork_on($target as usize, $entry as usize, &args)
}};
($target:expr, $entry:ident, $($arg:expr),*) => {{
    let args = [$($arg as usize,)*];
    crate::_fork_on($target as usize, $entry as usize, &args)
}};
}

#[macro_export]
macro_rules! try_fork_on {
($target:expr, $entry:ident) => {{
    let args:[usize;0] = [0;0];
    crate::_try_fork_on($target as usize, $entry as usize, &args)
}};
($target:expr, $entry:ident, $($arg:expr),*) => {{
    let args = [$($arg as usize,)*];
    crate::_try_fork_on($target as usize, $entry as usize, &args)
}};
}
