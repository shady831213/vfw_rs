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
    use crate::arch::arch;
    use crate::TaskId;
    use core::sync::atomic::{AtomicU16, Ordering};

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
                if let Some(id) = arch::_try_fork_on(i, task_id, entry, args_len, args) {
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
            if let Some(id) = arch::_try_fork_on(target_id, task_id, entry, args_len, args) {
                break id.raw();
            }
            core::hint::spin_loop();
        }
    }

    #[no_mangle]
    extern "C" fn c_try_fork(entry: usize, args_len: usize, args: *const usize) -> u32 {
        let task_id = get_task_id();
        for i in 1..crate::num_cores() {
            if let Some(id) = arch::_try_fork_on(i, task_id, entry, args_len, args) {
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
        if let Some(id) = arch::_try_fork_on(target_id, get_task_id(), entry, args_len, args) {
            return id.raw();
        }
        -1i32 as u32
    }

    #[inline]
    pub fn _fork(entry: usize, args: &[usize]) -> u32 {
        let task_id = get_task_id();
        loop {
            for i in 1..crate::num_cores() {
                if let Some(id) = arch::_try_fork_on(i, task_id, entry, args.len(), args.as_ptr()) {
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
            if let Some(id) =
                arch::_try_fork_on(target_id, task_id, entry, args.len(), args.as_ptr())
            {
                break id.raw();
            }
            core::hint::spin_loop();
        }
    }

    #[inline]
    pub fn _try_fork(entry: usize, args: &[usize]) -> Option<u32> {
        let task_id = get_task_id();
        for i in 1..crate::num_cores() {
            if let Some(id) = arch::_try_fork_on(i, task_id, entry, args.len(), args.as_ptr()) {
                return Some(id.raw());
            }
        }
        None
    }

    #[inline]
    pub fn _try_fork_on(target_id: usize, entry: usize, args: &[usize]) -> Option<u32> {
        arch::_try_fork_on(target_id, get_task_id(), entry, args.len(), args.as_ptr())
            .map(|id| id.raw())
    }

    #[no_mangle]
    pub extern "C" fn join(task_id: u32) {
        arch::_join(TaskId::from_u32(task_id as u32))
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
    #[inline]
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
