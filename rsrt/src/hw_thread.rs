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

static CPUS: [Mutex<Option<Task>>; MAX_CORES] =
    [const { Mutex::new(Option::<Task>::None) }; MAX_CORES];

fn schedule(hartid: usize) {
    send_others_ipis(hartid, 0xffffffff)
}

fn schedule_on(target_id: usize) {
    send_ipi(target_id).unwrap();
}

pub fn idle() {
    let hartid = hartid();
    loop {
        {
            let cur_task = CPUS[hartid].lock();
            if let Some(task) = cur_task.as_ref() {
                let t = *task;
                //should drop the lock first then send ipi
                drop(cur_task);
                (run_task().unwrap())(&t);
                *CPUS[hartid].lock() = None;
                schedule(hartid);
            } else {
                //should drop the lock before sleep
                drop(cur_task);
                //Allow cpu enter interrupt when goto sleep, if there's soft int, handler should ignore it. Then after go back from handler,
                //wfi can exit.
                //Besides, idle/fork/join should never called in interrupt context! So we don't need save/restore flag.
                // let flag = save_flag();
                wait_ipi(hartid);
                // restore_flag(flag);
            }
        }
    }
}

pub static EMPTY_ARGS: [usize; 0] = [0; 0];

#[no_mangle]
extern "C" fn c_fork(entry: usize, args_len: usize, args: *const usize) -> usize {
    let hartid = hartid();
    loop {
        let id = c_try_fork(entry, args_len, args);
        if id >= 0 {
            return id as usize;
        } else {
            // let flag = save_flag();
            wait_ipi(hartid);
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
    let hartid = hartid();
    loop {
        if let Ok(id) = __try_fork_on(target_id, entry, args_len, args) {
            return id;
        } else {
            // let flag = save_flag();
            wait_ipi(hartid);
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

pub fn fork(entry: usize, args: &[usize]) -> usize {
    let hartid = hartid();
    loop {
        if let Ok(id) = try_fork(entry, args) {
            return id;
        } else {
            // let flag = save_flag();
            wait_ipi(hartid);
            // restore_flag(flag);
        }
    }
}

pub fn fork_on(target_id: usize, entry: usize, args: &[usize]) -> usize {
    let hartid = hartid();
    loop {
        if let Ok(id) = try_fork_on(target_id, entry, args) {
            return id;
        } else {
            // let flag = save_flag();
            wait_ipi(hartid);
            // restore_flag(flag);
        }
    }
}

pub fn try_fork(entry: usize, args: &[usize]) -> Result<usize, &'static str> {
    for i in 1..num_cores() as usize {
        if let Ok(id) = try_fork_on(i, entry, args) {
            return Ok(id);
        }
    }
    Err("All cores are busy!")
}

pub fn try_fork_on(target_id: usize, entry: usize, args: &[usize]) -> Result<usize, &'static str> {
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
    let mut cur_task = CPUS[target_id].lock();
    if cur_task.as_ref().is_some() {
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
        *cur_task = Some(task);
        schedule_on(target_id);
        Ok(target_id)
    }
}

#[no_mangle]
pub extern "C" fn join(task_id: usize) {
    let hartid = hartid();
    loop {
        let cur_task = CPUS[task_id].lock();
        if cur_task.as_ref().is_some() {
            drop(cur_task);
            // let flag = save_flag();
            wait_ipi(hartid);
            // restore_flag(flag);
        } else {
            break;
        }
    }
}
