use crate::arch;
use crate::arch::FlowContext;
use crate::exit;
use crate::hsm::HsmCell;
use crate::hw_thread::{main_thread, thread_loop, Task, TaskId};
use crate::init_heap;
use crate::wait_ipi;
use crate::{Stack, VfwStack};
use core::ptr::NonNull;

//sections must be always 4 bytes aligned!!!
#[inline(never)]
#[link_section = ".init.rust"]
unsafe fn _sec_reloc(start: usize, end: usize, load_start: usize) {
    let size = (((end + 3) >> 2) << 2) - start;
    if size > 0 && start != load_start {
        for i in (0..size).step_by(core::mem::size_of::<u32>()) {
            *((start + i) as *mut u32) = *((load_start + i) as *const u32);
        }
    }
}

macro_rules! sec_reloc {
    ($start:ident, $end:ident, $load_start:ident) => {
        unsafe {
            extern "C" {
                static mut $start: u32;
                static $end: u32;
                static $load_start: u32;
            }
            _sec_reloc(
                &raw mut $start as *mut _ as usize,
                &raw const $end as *const _ as usize,
                &raw const $load_start as *const _ as usize,
            )
        }
    };
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
mod lottery {
    pub(super) const REL_INIT: usize = 0;
    pub(super) const REL_ACQ: usize = 1;
    pub(super) const REL_DONE: usize = 2;

    use core::sync::atomic::AtomicUsize;

    #[link_section = ".rel_lottary"]
    pub(super) static REL_LOTTARY: AtomicUsize = AtomicUsize::new(REL_INIT);
}

#[inline(always)]
#[cfg(not(feature = "load_bss"))]
fn __init_bss(s: *mut u8, n: usize) {
    unsafe {
        for i in 0..n {
            *((s as usize + i) as *mut u8) = 0;
        }
    };
}

#[cfg(feature = "load_bss")]
fn __init_bss(_s: *mut u8, _n: usize) {}

#[inline(always)]
fn init_bss() {
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;
        static mut _s_synced_bss: u8;
        static mut _e_synced_bss: u8;
    }
    let m_sbss = &raw mut _sbss as *mut _ as usize;
    let m_ebss = &raw mut _ebss as *mut _ as usize;
    let size = m_ebss - m_sbss;
    if size > 0 {
        __init_bss(m_sbss as *mut u8, size);
    }
    let m_sbss = &raw mut _s_synced_bss as *mut _ as usize;
    let m_ebss = &raw mut _e_synced_bss as *mut _ as usize;
    let size = m_ebss - m_sbss;
    if size > 0 {
        __init_bss(m_sbss as *mut u8, size);
    }
}

#[inline(always)]
fn init_cpu_bss() {
    extern "C" {
        static mut _s_cpu_bss: u8;
        static mut _e_cpu_bss: u8;
    }
    let m_sbss = &raw mut _s_cpu_bss as *mut _ as usize;
    let m_ebss = &raw mut _e_cpu_bss as *mut _ as usize;
    let size = m_ebss - m_sbss;
    if size > 0 {
        __init_bss(m_sbss as *mut u8, size);
    }
}

#[inline(always)]
fn reset_ctxs() {
    unsafe {
        for mut c in CPU_CTXS {
            c.reset()
        }
    }
}

#[inline(always)]
#[link_section = ".init.rust"]
pub(crate) fn vfw_relocation() {
    __pre_init();
    #[cfg(any(
        feature = "max_cores_128",
        feature = "max_cores_64",
        feature = "max_cores_32",
        feature = "max_cores_16",
        feature = "max_cores_8",
        feature = "max_cores_4",
        feature = "max_cores_2"
    ))]
    {
        use core::hint::spin_loop;
        use core::sync::atomic::Ordering;
        loop {
            match lottery::REL_LOTTARY.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Relaxed) {
                Ok(_) => {
                    sec_reloc!(_srodata, _erodata, _srodata_load);
                    sec_reloc!(_stext, _etext, _stext_load);
                    sec_reloc!(_sdata, _edata, _sdata_load);
                    sec_reloc!(_s_synced_data, _e_synced_data, _s_synced_data_load);
                    sec_reloc!(_s_cpu_data, _e_cpu_data, _s_cpu_data_load);
                    init_cpu_bss();
                    init_bss();
                    reset_ctxs();
                    lottery::REL_LOTTARY.store(lottery::REL_DONE, Ordering::Release);
                    break;
                }
                Err(s) => match s {
                    lottery::REL_ACQ => spin_loop(),
                    lottery::REL_DONE => {
                        #[cfg(not(feature = "cpu_data_non_priv"))]
                        {
                            sec_reloc!(_s_cpu_data, _e_cpu_data, _s_cpu_data_load);
                            init_cpu_bss();
                        }
                        break;
                    }
                    _ => lottery::REL_LOTTARY.store(lottery::REL_INIT, Ordering::Release),
                },
            }
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
    {
        sec_reloc!(_srodata, _erodata, _srodata_load);
        sec_reloc!(_stext, _etext, _stext_load);
        sec_reloc!(_sdata, _edata, _sdata_load);
        sec_reloc!(_s_synced_data, _e_synced_data, _s_synced_data_load);
        sec_reloc!(_s_cpu_data, _e_cpu_data, _s_cpu_data_load);
        init_cpu_bss();
        init_bss();
        reset_ctxs();
    }
}

#[no_mangle]
pub extern "C" fn num_cores() -> usize {
    extern "C" {
        static _num_cores: u8;
    }
    let m_num_cores = &raw const _num_cores as *const u8 as usize;
    m_num_cores
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

#[linkage = "weak"]
#[no_mangle]
#[link_section = ".init.rust"]
extern "C" fn __pre_init() {}

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
        main_thread(0, vfw_main as usize, &[]);
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

    fn reset(&mut self) {
        self.trap = FlowContext::ZERO;
        self.hsm = HsmCell::new();
        self.current = TaskId::new(0, 0);
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
