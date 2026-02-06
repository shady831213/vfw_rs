use crate::arch::FlowContext;
use crate::hsm::HsmCell;
use crate::hw_thread::{main_thread, thread_loop, Task, TaskId};
use crate::*;
use core::ptr::NonNull;

#[linkage = "weak"]
#[no_mangle]
extern "C" fn __post_init() {}

pub(crate) fn vfw_start() {
    if hartid() >= num_cores() || hartid() >= MAX_CORES {
        loop {
            wait_ipi();
        }
    }
    extern "C" {
        fn __boot_core_init();
    }
    VfwStack.load_context(cpu_ctx(hartid()).context_ptr(), arch::trap_handler);
    __post_init();
    if hartid() == 0 {
        init_heap();
        if num_cores() > MAX_CORES {
            panic!(
                "num_cores({}) > MAX_CORES({})! please check link script and features!",
                num_cores(),
                MAX_CORES
            );
        }
        unsafe {
            __boot_core_init();
        }
        main_thread(0, vfw_main as *const () as usize, &[]);
    }
    thread_loop();
}

pub fn per_cpu_offset() -> usize {
    if PER_CPU_LEN > 1 {
        hartid()
    } else {
        0
    }
}

#[inline]
fn vfw_main() -> ! {
    extern "C" {
        fn main() -> u32;
    }
    let ret = unsafe { main() };
    exit(ret);
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

#[inline(always)]
pub(crate) fn cpu_ctx(hartid: usize) -> &'static mut HartContext {
    unsafe { &mut CPU_CTXS[hartid] }
}
