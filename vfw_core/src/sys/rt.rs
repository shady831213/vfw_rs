use crate::arch::arch;
use crate::exit;
use crate::hsm::HsmCell;
use crate::hw_thread::get_task_id;
use crate::init_heap;
use crate::msg::MsgCell;
use crate::Stack;
use crate::{clear_ipi, send_ipi, wait_ipi};
use core::ptr::NonNull;
use fast_trap::FlowContext;

#[no_mangle]
pub extern "C" fn num_cores() -> usize {
    extern "C" {
        static _num_cores: u8;
        static _provide_base: usize;
    }
    let m_num_cores = (unsafe { &_num_cores }) as *const u8 as usize;
    let m_provide_base = unsafe { &_provide_base } as *const usize as usize;
    m_num_cores - m_provide_base
}

#[no_mangle]
pub extern "C" fn hartid() -> usize {
    arch::hartid()
}

#[no_mangle]
pub extern "C" fn save_flag() -> usize {
    arch::save_flag()
}

#[no_mangle]
pub extern "C" fn restore_flag(flag: usize) {
    arch::restore_flag(flag)
}

fn init_bss() {
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
pub const MAX_CORES: usize = 2;
#[cfg(feature = "max_cores_4")]
pub const MAX_CORES: usize = 4;
#[cfg(feature = "max_cores_8")]
pub const MAX_CORES: usize = 8;
#[cfg(feature = "max_cores_16")]
pub const MAX_CORES: usize = 16;
#[cfg(feature = "max_cores_32")]
pub const MAX_CORES: usize = 32;
#[cfg(feature = "max_cores_64")]
pub const MAX_CORES: usize = 64;
#[cfg(feature = "max_cores_128")]
pub const MAX_CORES: usize = 128;

#[cfg(not(any(
    feature = "max_cores_128",
    feature = "max_cores_64",
    feature = "max_cores_32",
    feature = "max_cores_16",
    feature = "max_cores_8",
    feature = "max_cores_4",
    feature = "max_cores_2"
)))]
pub const MAX_CORES: usize = 1;

#[export_name = "vfw_start"]
fn vfw_start() {
    if hartid() >= num_cores() || hartid() >= MAX_CORES {
        loop {
            wait_ipi();
        }
    }
    extern "C" {
        fn __boot_core_init();
    }
    extern "C" {
        fn __pre_init();
    }
    Stack.load_as_stack(cpu_ctx(hartid()).context_ptr(), arch::vfw_fast_handler);
    arch::init_fast_trap();
    unsafe { __pre_init() };
    if hartid() == 0 {
        init_bss();
        init_heap();
        if num_cores() > MAX_CORES {
            panic!(
                "num_cores({}) > MAX_CORES({})! please check link script and features!",
                num_cores(),
                MAX_CORES
            );
        }
        unsafe { __boot_core_init() };
        let mut _a0: usize = 0;
        fork_call(
            &mut _a0,
            0,
            get_task_id() as usize,
            vfw_main as usize,
            0,
            0,
            0,
        );
    }
    unsafe {
        fast_trap::trap_entry();
    }
}

pub(crate) const IPI_HSM: usize = -1isize as usize;

#[repr(usize)]
pub(crate) enum VfwCall {
    Fork = 0,
    Join,
}
impl core::convert::TryFrom<usize> for VfwCall {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(VfwCall::Fork),
            1 => Ok(VfwCall::Join),
            v => Err(v),
        }
    }
}

#[inline(always)]
pub(crate) fn vfw_idle() {
    wait_ipi();
    unsafe {
        if *cpu_ctx(hartid()).ipi_msg.local().recv() == IPI_HSM {
            clear_ipi(hartid());
            cpu_ctx(hartid()).ipi_msg.local().done();
        }
    }
}

#[inline(always)]
pub(crate) fn vfw_call(a: &mut [usize; 8]) -> usize {
    let hartid = hartid();
    let a_in: [usize; 8] = *a;
    match VfwCall::try_from(a_in[0]) {
        Ok(VfwCall::Fork) => fork_call(
            &mut a[0], a_in[1], a_in[2], a_in[3], a_in[4], a_in[5], hartid,
        ),
        Ok(VfwCall::Join) => join_call(a_in[1]),
        Err(e) => panic!("Invalid VfwCall {:#x}", e),
    }
}

// boot sp can not include handler call stack
#[inline(always)]
pub(crate) fn fork_call(
    a0: &mut usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    hartid: usize,
) -> usize {
    //fork on other core
    let hart_target = a1;
    if hart_target >= num_cores() {
        panic!("Invalid fork target id {}!", hart_target);
    }
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
    let ret = cpu_ctx(hart_target).hsm.remote().start(task);
    if ret {
        cpu_ctx(hart_target).ipi_msg.remote().send(IPI_HSM);
        send_ipi(hart_target);
    }
    *a0 = ret as usize;
    1
}

// boot sp can not include handler call stack
#[inline(always)]
pub(crate) fn join_call(a1: usize) -> usize {
    #[inline(always)]
    fn finished(issued: u16, retired: u16) -> bool {
        if (issued >> 15) != (retired >> 15) {
            retired <= issued
        } else {
            retired >= issued
        }
    }
    let id = TaskId::from_u32(a1 as u32);
    let hart_target = id.hart_id() as usize;
    if hart_target >= num_cores() {
        panic!("Invalid join target id {}!", hart_target);
    }
    let cpu = cpu_ctx(hart_target);
    loop {
        if cpu.hsm.remote().get_status().expect("Invalid State!") == crate::hsm::HsmState::Stopped
            && finished(id.task_id(), cpu.current.task_id())
        {
            break;
        }
        core::hint::spin_loop();
    }
    0
}

pub fn per_cpu_offset() -> usize {
    if PER_CPU_LEN > 1 {
        hartid()
    } else {
        0
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

pub(crate) struct HartContext {
    pub(crate) trap: FlowContext,
    pub(crate) hsm: HsmCell<Task>,
    pub(crate) ipi_msg: MsgCell<usize>,
    pub(crate) current: TaskId,
}

impl HartContext {
    const fn new() -> Self {
        HartContext {
            trap: FlowContext::ZERO,
            hsm: HsmCell::new(),
            ipi_msg: MsgCell::new(0),
            current: TaskId::new(0, 0),
        }
    }

    #[inline]
    pub(crate) fn context_ptr(&mut self) -> NonNull<FlowContext> {
        unsafe { NonNull::new_unchecked(&mut self.trap) }
    }
}

#[link_section = ".synced.data"]
static mut CPU_CTXS: [HartContext; MAX_CORES] = [const { HartContext::new() }; MAX_CORES];

#[inline(always)]
pub(crate) fn cpu_ctx(hartid: usize) -> &'static mut HartContext {
    unsafe { &mut CPU_CTXS[hartid] }
}
