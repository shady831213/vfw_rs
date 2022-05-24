#[cfg(not(feature = "max_cores_1"))]
mod hw_thread_imp {
    use crate::arch::*;
    use crate::ipi::*;
    use crate::sys::num_cores;
    use spin::Mutex;
    #[cfg(feature = "max_cores_4")]
    const MAX_CORES: usize = 4;
    #[cfg(feature = "max_cores_8")]
    const MAX_CORES: usize = 8;
    #[cfg(feature = "max_cores_16")]
    const MAX_CORES: usize = 16;
    #[cfg(feature = "max_cores_32")]
    const MAX_CORES: usize = 32;
    #[cfg(feature = "max_cores_64")]
    const MAX_CORES: usize = 64;
    #[cfg(feature = "max_cores_128")]
    const MAX_CORES: usize = 128;

    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Task {
        pub entry: usize,
        pub args: [usize; 8],
    }

    #[derive(Copy, Clone)]
    #[repr(C)]
    struct CpuState {
        task: Option<Task>,
        cur_cnt: u16,
        finish_cnt: u16,
    }
    impl CpuState {
        const fn empty() -> CpuState {
            CpuState {
                task: None,
                cur_cnt: 0,
                finish_cnt: 0,
            }
        }
        fn is_done(&self, id: u16) -> bool {
            if (id & 0x8000) != (self.finish_cnt & 0x8000) {
                self.finish_cnt < id
            } else {
                self.finish_cnt > id
            }
        }
    }

    #[link_section = ".synced.bss"]
    static CPUS: [Mutex<CpuState>; MAX_CORES] =
        [const { Mutex::new(CpuState::empty()) }; MAX_CORES];

    union ArchRunTask {
        handler: fn(task: &Task),
        reserved: usize,
    }
    #[repr(C)]
    struct ArchHandlers {
        run_task: ArchRunTask,
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
    #[link_section = ".synced.bss"]
    static mut ARCH: ArchHandlers = ArchHandlers {
        run_task: ArchRunTask { reserved: 0 },
    };

    fn schedule(hartid: usize) {
        send_others_ipis(hartid, 0xffffffff);
    }

    fn schedule_on(target_id: usize) {
        send_ipi(target_id).unwrap();
    }

    pub fn idle() {
        let hartid = hartid();
        loop {
            {
                let cpu = CPUS[hartid].lock();
                if let Some(task) = cpu.task.as_ref() {
                    let t = *task;
                    //should drop the lock first then send ipi
                    drop(cpu);
                    (run_task().unwrap())(&t);
                    {
                        let mut cpu = CPUS[hartid].lock();
                        cpu.finish_cnt = cpu.finish_cnt.wrapping_add(1);
                        cpu.task = None;
                    }
                    schedule(hartid);
                } else {
                    //should drop the lock before sleep
                    drop(cpu);
                    //Allow cpu enter interrupt when goto sleep, if there's soft int, handler should ignore it. Then after go back from handler,
                    //wfi can exit.
                    //Besides, idle/fork/join should never called in interrupt context! So we don't need save/restore flag.
                    // let flag = save_flag();
                    wait_ipi();
                    // restore_flag(flag);
                }
            }
        }
    }

    #[no_mangle]
    extern "C" fn c_fork(entry: usize, args_len: usize, args: *const usize) -> usize {
        loop {
            let id = c_try_fork(entry, args_len, args);
            if id >= 0 {
                return id as usize;
            } else {
                // let flag = save_flag();
                wait_ipi();
                // restore_flag(flag);
            }
        }
    }

    #[no_mangle]
    extern "C" fn c_fork_on(
        target_id: usize,
        entry: usize,
        args_len: usize,
        args: *const usize,
    ) -> usize {
        loop {
            if let Ok(id) = __try_fork_on(target_id, entry, args_len, args) {
                return id;
            } else {
                // let flag = save_flag();
                wait_ipi();
                // restore_flag(flag);
            }
        }
    }

    #[no_mangle]
    extern "C" fn c_try_fork(entry: usize, args_len: usize, args: *const usize) -> isize {
        for i in 1..num_cores() as usize {
            if let Ok(id) = __try_fork_on(i, entry, args_len, args) {
                return id as isize;
            }
        }
        -1
    }

    #[no_mangle]
    extern "C" fn c_try_fork_on(
        target_id: usize,
        entry: usize,
        args_len: usize,
        args: *const usize,
    ) -> isize {
        if let Ok(id) = __try_fork_on(target_id, entry, args_len, args) {
            return id as isize;
        }
        -1
    }

    pub fn _fork(entry: usize, args: &[usize]) -> usize {
        loop {
            if let Ok(id) = _try_fork(entry, args) {
                return id;
            } else {
                // let flag = save_flag();
                wait_ipi();
                // restore_flag(flag);
            }
        }
    }

    pub fn _fork_on(target_id: usize, entry: usize, args: &[usize]) -> usize {
        loop {
            if let Ok(id) = _try_fork_on(target_id, entry, args) {
                return id;
            } else {
                // let flag = save_flag();
                wait_ipi();
                // restore_flag(flag);
            }
        }
    }

    pub fn _try_fork(entry: usize, args: &[usize]) -> Result<usize, &'static str> {
        for i in 1..num_cores() as usize {
            if let Ok(id) = _try_fork_on(i, entry, args) {
                return Ok(id);
            }
        }
        Err("All cores are busy!")
    }

    pub fn _try_fork_on(
        target_id: usize,
        entry: usize,
        args: &[usize],
    ) -> Result<usize, &'static str> {
        __try_fork_on(target_id, entry, args.len(), args.as_ptr())
    }

    fn __try_fork_on(
        target_id: usize,
        entry: usize,
        args_len: usize,
        args: *const usize,
    ) -> Result<usize, &'static str> {
        if args_len > 8 {
            return Err("number of args is larger than max args length 8!");
        }
        let mut cpu = CPUS[target_id].lock();
        if cpu.task.as_ref().is_some() {
            Err("busy!")
        } else {
            let mut task = Task {
                entry,
                args: [0; 8],
            };
            for i in 0..args_len {
                task.args[i] = unsafe {
                    *((args as usize + i * core::mem::size_of::<usize>() as usize) as *const usize)
                };
            }
            let task_id = (target_id << 16) | cpu.cur_cnt as usize;
            cpu.cur_cnt = cpu.cur_cnt.wrapping_add(1);
            cpu.task = Some(task);
            schedule_on(target_id);
            Ok(task_id)
        }
    }

    #[no_mangle]
    pub extern "C" fn join(task_id: usize) {
        let target_id = task_id >> 16;
        let seq_id = task_id as u16;
        loop {
            let cpu = CPUS[target_id].lock();
            if cpu.is_done(seq_id) {
                break;
            } else {
                drop(cpu);
                wait_ipi();
            }
        }
    }
}

#[cfg(feature = "max_cores_1")]
mod hw_thread_stub {
    pub fn idle() {
        panic!("hw_thread not support!")
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
    ) -> usize {
        panic!("hw_thread not support!")
    }
    #[no_mangle]
    extern "C" fn c_try_fork(_entry: usize, _args_len: usize, _args: *const usize) -> isize {
        panic!("hw_thread not support!")
    }
    #[no_mangle]
    extern "C" fn c_try_fork_on(
        _target_id: usize,
        _entry: usize,
        _args_len: usize,
        _args: *const usize,
    ) -> isize {
        panic!("hw_thread not support!")
    }
    pub fn _fork(_entry: usize, _args: &[usize]) -> usize {
        panic!("hw_thread not support!")
    }
    pub fn _fork_on(_target_id: usize, _entry: usize, _args: &[usize]) -> usize {
        panic!("hw_thread not support!")
    }
    pub fn _try_fork(_entry: usize, _args: &[usize]) -> Result<usize, &'static str> {
        panic!("hw_thread not support!")
    }

    pub fn _try_fork_on(
        _target_id: usize,
        _entry: usize,
        _args: &[usize],
    ) -> Result<usize, &'static str> {
        panic!("hw_thread not support!")
    }
    #[no_mangle]
    pub extern "C" fn join(_task_id: usize) {
        panic!("hw_thread not support!")
    }
}
#[cfg(not(feature = "max_cores_1"))]
pub use hw_thread_imp::*;
#[cfg(feature = "max_cores_1")]
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
