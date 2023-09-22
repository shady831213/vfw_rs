use super::arch::hartid;
use crate::exit;
use crate::hsm::HsmCell;
use crate::hw_thread::{Task, TaskId};
use crate::init_heap;
use crate::stack::*;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicU16, Ordering};
use fast_trap::{FastContext, FastResult, FlowContext};
pub(crate) fn init_bss() {
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;
        fn __init_bss(s: *mut u8, n: usize);
    }
    let m_sbss = unsafe { &mut _sbss } as *mut _ as usize;
    let m_ebss = unsafe { &mut _ebss } as *mut _ as usize;
    let size = m_ebss - m_sbss;
    if size > 0 {
        unsafe {
            __init_bss(m_sbss as *mut u8, size);
        }
    }
}

#[cfg(all(feature = "cpu_data_non_priv", feature = "max_cores_128"))]
pub const PER_CPU_LEN: usize = 128;
#[cfg(all(feature = "cpu_data_non_priv", feature = "max_cores_64"))]
pub const PER_CPU_LEN: usize = 64;
#[cfg(all(feature = "cpu_data_non_priv", feature = "max_cores_32"))]
pub const PER_CPU_LEN: usize = 32;
#[cfg(all(feature = "cpu_data_non_priv", feature = "max_cores_16"))]
pub const PER_CPU_LEN: usize = 16;
#[cfg(all(feature = "cpu_data_non_priv", feature = "max_cores_8"))]
pub const PER_CPU_LEN: usize = 8;
#[cfg(all(feature = "cpu_data_non_priv", feature = "max_cores_4"))]
pub const PER_CPU_LEN: usize = 4;
#[cfg(all(feature = "cpu_data_non_priv", feature = "max_cores_2"))]
pub const PER_CPU_LEN: usize = 2;
#[cfg(any(
    not(feature = "cpu_data_non_priv"),
    not(any(
        feature = "max_cores_128",
        feature = "max_cores_64",
        feature = "max_cores_32",
        feature = "max_cores_16",
        feature = "max_cores_8",
        feature = "max_cores_4",
        feature = "max_cores_2"
    ))
))]
pub const PER_CPU_LEN: usize = 1;

#[cfg(feature = "max_cores_2")]
const MAX_CORES: usize = 2;
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

#[cfg(not(any(
    feature = "max_cores_128",
    feature = "max_cores_64",
    feature = "max_cores_32",
    feature = "max_cores_16",
    feature = "max_cores_8",
    feature = "max_cores_4",
    feature = "max_cores_2"
)))]
const MAX_CORES: usize = 1;

#[repr(usize)]
pub(crate) enum VfwCall {
    Fork = 0,
    Join,
}

// boot sp can not include handler call stack
#[inline(always)]
pub(crate) fn __fork_call(
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    hartid: usize,
) -> bool {
    //fork on other core
    let hart_target = a1;
    let task_id = a2 as u16;
    let entry = a3;
    let arg_len = a4;
    let args = a5;
    let mut task = Task {
        entry,
        args: [0; 8],
        task_id: TaskId::new(hartid as u16, task_id),
    };
    for i in 0..arg_len {
        unsafe { task.args[i] = *((args + i * core::mem::size_of::<usize>()) as *const usize) };
    }
    cpu_ctx(hart_target).hsm.remote().start(task)
}

// boot sp can not include handler call stack
#[inline(always)]
pub(crate) fn __join_call(a1: usize) {
    #[inline]
    fn finished(issued: u16, retired: u16) -> bool {
        if (issued >> 15) != (retired >> 15) {
            retired <= issued
        } else {
            retired >= issued
        }
    }
    let id = TaskId::from_u32(a1 as u32);
    let cpu = cpu_ctx(id.hart_id() as usize);
    loop {
        if cpu.hsm.remote().get_status().expect("Invalid State!") == crate::hsm::HsmState::Stopped
            && finished(id.task_id(), cpu.current.task_id())
        {
            break;
        }
        core::hint::spin_loop();
    }
}

pub fn per_cpu_offset() -> usize {
    if PER_CPU_LEN > 1 {
        hartid()
    } else {
        0
    }
}
#[inline]
fn try_fork_on_wrapper(
    hart_target: usize,
    task_id: u16,
    entry: usize,
    arg_len: usize,
    args: &[usize],
) -> Option<TaskId> {
    extern "C" {
        fn try_fork_on(
            hart_target: usize,
            task_id: u16,
            entry: usize,
            arg_len: usize,
            args: *const usize,
        ) -> usize;
    }
    let ret = unsafe { try_fork_on(hart_target, task_id, entry, arg_len, args.as_ptr()) };
    if ret == 0 {
        None
    } else {
        Some(TaskId::new(hart_target as u16, task_id))
    }
}

pub fn new_try_fork_on(
    hart_target: usize,
    entry: usize,
    arg_len: usize,
    args: &[usize],
) -> Option<TaskId> {
    try_fork_on_wrapper(
        hart_target,
        HW_TIDS.fetch_add(1, Ordering::SeqCst),
        entry,
        arg_len,
        args,
    )
}

pub fn new_fork_on(hart_target: usize, entry: usize, arg_len: usize, args: &[usize]) -> TaskId {
    let task_id = HW_TIDS.fetch_add(1, Ordering::SeqCst);
    loop {
        if let Some(id) = try_fork_on_wrapper(hart_target, task_id, entry, arg_len, args) {
            break id;
        }
        core::hint::spin_loop();
    }
}

pub fn new_fork(entry: usize, arg_len: usize, args: &[usize]) -> TaskId {
    let task_id = HW_TIDS.fetch_add(1, Ordering::SeqCst);
    loop {
        for i in 1..crate::num_cores() {
            if let Some(id) = try_fork_on_wrapper(i, task_id, entry, arg_len, args) {
                return id;
            }
        }
        core::hint::spin_loop();
    }
}

#[inline]
pub(crate) fn vfw_main() -> ! {
    extern "C" {
        fn main() -> u32;
    }
    let ret = unsafe { main() };
    exit(ret);
}

#[inline]
pub(crate) fn vfw_done() {
    unsafe {
        cpu_ctx(hartid()).hsm.local().stop();
        fast_trap::trap_entry();
    }
}

pub(crate) struct HartContext {
    pub(crate) trap: FlowContext,
    pub(crate) hsm: HsmCell<Task>,
    pub(crate) current: TaskId,
}

impl HartContext {
    const fn new() -> Self {
        HartContext {
            trap: FlowContext::ZERO,
            hsm: HsmCell::new(),
            current: TaskId::new(0, 0),
        }
    }

    #[inline]
    pub(crate) fn context_ptr(&mut self) -> NonNull<FlowContext> {
        unsafe { NonNull::new_unchecked(&mut self.trap) }
    }
}

#[link_section = ".synced.bss"]
static mut CPU_CTXS: [HartContext; MAX_CORES] = [const { HartContext::new() }; MAX_CORES];
#[link_section = ".synced.bss"]
static HW_TIDS: AtomicU16 = AtomicU16::new(0);

#[inline(always)]
pub(crate) fn cpu_ctx(hartid: usize) -> &'static mut HartContext {
    unsafe { &mut CPU_CTXS[hartid] }
}
