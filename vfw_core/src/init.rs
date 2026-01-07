use crate::arch;
use crate::hartid;

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
    pub(super) use core::sync::atomic::AtomicUsize;
    pub(super) use core::sync::atomic::Ordering;

    #[cfg(feature = "multicores_init")]
    mod lottery_core {
        use core::sync::atomic::AtomicUsize;
        #[link_section = ".rel_lottery"]
        pub static REL_LOTTERY_CORE: AtomicUsize = AtomicUsize::new(0);
    }

    #[cfg(feature = "multicores_init")]
    pub(super) use lottery_core::*;

    #[link_section = ".rel_lottery"]
    pub(super) static REL_WINNER_CORE: AtomicUsize = AtomicUsize::new(0);
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

//inline never to keep got access after regloc_got
#[inline(never)]
#[link_section = ".init.rust"]
fn winner_init_job() {
    sec_reloc!(_srodata, _erodata, _srodata_load);
    sec_reloc!(_stext, _etext, _stext_load);
    sec_reloc!(_sdata, _edata, _sdata_load);
    sec_reloc!(_s_synced_data, _e_synced_data, _s_synced_data_load);
    init_bss();
    sec_reloc!(_s_cpu_data, _e_cpu_data, _s_cpu_data_load);
    init_cpu_bss();
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
#[link_section = ".init.rust"]
fn winner_init() {
    #[cfg(feature = "multicores_init")]
    {
        if lottery::REL_LOTTERY_CORE.load(lottery::Ordering::Acquire) % arch::init_num_cores()
            == hartid()
        {
            arch::reloc_got();
            winner_init_job();
            lottery::REL_LOTTERY_CORE.store(
                (hartid() + 1) % arch::init_num_cores(),
                lottery::Ordering::Release,
            );
            lottery::REL_WINNER_CORE.store(hartid(), lottery::Ordering::Release);
        }
    }
    //if single core init, first core do this job natrually
    #[cfg(not(feature = "multicores_init"))]
    {
        arch::reloc_got();
        winner_init_job();
        lottery::REL_WINNER_CORE.store(hartid(), lottery::Ordering::Release);
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
#[link_section = ".init.rust"]
fn loser_init_job() {
    #[cfg(not(feature = "cpu_data_non_priv"))]
    {
        sec_reloc!(_s_cpu_data, _e_cpu_data, _s_cpu_data_load);
        init_cpu_bss();
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
#[link_section = ".init.rust"]
fn losers_init() {
    #[cfg(feature = "multicores_init")]
    {
        loop {
            let cur_core =
                lottery::REL_LOTTERY_CORE.load(lottery::Ordering::Acquire) % arch::init_num_cores();
            if cur_core == hartid() {
                if lottery::REL_WINNER_CORE.load(lottery::Ordering::Acquire) != hartid() {
                    loser_init_job();
                    lottery::REL_LOTTERY_CORE.store(
                        (hartid() + 1) % arch::init_num_cores(),
                        lottery::Ordering::Release,
                    );
                }
                break;
            }
        }
    }
    #[cfg(not(feature = "multicores_init"))]
    {
        if lottery::REL_WINNER_CORE.load(lottery::Ordering::Acquire) != hartid() {
            loser_init_job();
        }
    }
}
#[linkage = "weak"]
#[no_mangle]
#[link_section = ".init.rust"]
extern "C" fn __pre_init() {}

//after we are done with got, pic can work
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
        winner_init();
        losers_init();
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
        arch::reloc_got();
        winner_init_job();
    }
}
