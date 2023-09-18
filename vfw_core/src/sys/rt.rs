use super::arch::hartid;
use crate::exit;
use crate::hsm::HsmCell;
use crate::init_heap;
use core::ptr::NonNull;
use fast_trap::{FastContext, FastResult, FlowContext, FreeTrapStack};
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

pub fn per_cpu_offset() -> usize {
    if PER_CPU_LEN > 1 {
        hartid()
    } else {
        0
    }
}

extern "C" {
    static mut _sstack: u8;
    static _stack_size: usize;
    static _provide_base: usize;
}

pub struct Stack;
impl Stack {
    fn start(&self) -> usize {
        stack_start()
    }

    fn size(&self) -> usize {
        stack_size()
    }

    fn end(&self) -> usize {
        self.start() + self.size()
    }

    fn load_as_stack(&self) {
        // extern "C" {
        //     fn fast_handler(
        //         mut ctx: FastContext,
        //         a1: usize,
        //         a2: usize,
        //         a3: usize,
        //         a4: usize,
        //         a5: usize,
        //         a6: usize,
        //         a7: usize,
        //     ) -> FastResult;
        // }
        let context_ptr = unsafe { &mut CPU_CTXS[hartid()] }.context_ptr();
        core::mem::forget(
            FreeTrapStack::new(self.start()..self.end(), |_| {}, context_ptr, fast_handler)
                .unwrap()
                .load(),
        );
    }
}

#[no_mangle]
pub extern "C" fn stack_start() -> usize {
    let m_sstack = unsafe { &mut _sstack } as *mut _ as usize;
    #[cfg(feature = "stack_non_priv")]
    {
        m_sstack + stack_size() * hartid()
    }
    #[cfg(not(feature = "stack_non_priv"))]
    {
        m_sstack
    }
}

#[no_mangle]
pub extern "C" fn stack_size() -> usize {
    let m_stack_size = unsafe { &_stack_size } as *const usize as usize;
    let m_provide_base = unsafe { &_provide_base } as *const usize as usize;
    m_stack_size - m_provide_base
}

// #[export_name = "vfw_start"]
// fn vfw_start() {
//     use crate::hw_thread::idle;
//     extern "C" {
//         fn main() -> u32;
//     }
//     extern "C" {
//         fn __boot_core_init();
//     }
//     extern "C" {
//         fn __pre_init();
//     }
//     unsafe { __pre_init() };
//     let hartid = hartid();
//     if hartid == 0 {
//         init_bss();
//         init_heap();
//         unsafe { __boot_core_init() };
//         let ret = unsafe { main() };
//         exit(ret);
//     } else {
//         idle()
//     }
// }

#[export_name = "vfw_start"]
fn vfw_start() {
    use crate::hw_thread::idle;
    extern "C" {
        fn __boot_core_init();
    }
    extern "C" {
        fn __pre_init();
    }
    unsafe { __pre_init() };
    let hartid = hartid();
    if hartid == 0 {
        init_bss();
        init_heap();
        Stack.load_as_stack();
        // unsafe { __boot_core_init() };
        // let ret = unsafe { main() };
        // exit(ret);
        unsafe { &mut CPU_CTXS[hartid] }
            .hsm
            .remote()
            .start(Entry(__main as usize));
    } else {
        Stack.load_as_stack();
        // idle()
    }
    unsafe {
        fast_trap::trap_entry();
    }
}

fn __main() -> ! {
    extern "C" {
        fn main() -> u32;
    }
    let ret = unsafe { main() };
    exit(ret);
}
extern "C" fn fast_handler(
    mut ctx: FastContext,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
) -> FastResult {
    #[inline]
    fn boot(mut ctx: FastContext, start_addr: usize) -> FastResult {
        ctx.regs().a[0] = hartid();
        ctx.regs().pc = start_addr;
        ctx.call(2)
    }
    loop {
        match unsafe { &mut CPU_CTXS[hartid()].hsm.local().start() } {
            Ok(entry) => {
                break boot(ctx, entry.0);
            }
            Err(crate::hsm::HsmState::Stopped) => {}
            _ => panic!("stopped with unsupported trap"),
        }
    }
}

pub struct Entry(usize);

pub struct HartContext {
    trap: FlowContext,
    hsm: HsmCell<Entry>,
}

impl HartContext {
    const fn new() -> Self {
        HartContext {
            trap: FlowContext::ZERO,
            hsm: HsmCell::new(),
        }
    }
    #[inline]
    fn init(&mut self) {
        self.hsm = HsmCell::new();
    }

    #[inline]
    fn context_ptr(&mut self) -> NonNull<FlowContext> {
        unsafe { NonNull::new_unchecked(&mut self.trap) }
    }
}

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

#[link_section = ".synced.bss"]
static mut CPU_CTXS: [HartContext; MAX_CORES] = [const { HartContext::new() }; MAX_CORES];
