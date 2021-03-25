use crate::arch::*;
use alloc::alloc::GlobalAlloc;
use alloc::string::ToString;
use buddy_system_allocator::LockedHeap;
use core::alloc::Layout;
use core::fmt;
use core::ops::Deref;
use core::panic::PanicInfo;
extern "C" {
    fn __exit(code: u32) -> !;
}

#[no_mangle]
pub extern "C" fn exit(code: u32) -> ! {
    unsafe { __exit(code) }
}

union PrintStrHandler {
    handler: fn(s: &str),
    reserved: usize,
}

static mut PRINT_STR: PrintStrHandler = PrintStrHandler { reserved: 0 };

pub fn init_print_str(f: fn(s: &str)) {
    unsafe {
        PRINT_STR.handler = f;
    }
}

pub fn __print_str(s: &str) {
    unsafe {
        if PRINT_STR.reserved == 0 {
            return;
        }
        (PRINT_STR.handler)(s)
    }
}
pub fn __print(args: fmt::Arguments) {
    __print_str(&args.to_string())
}

#[macro_export]
macro_rules! print {
    ($fmt:expr) => (crate::__print_str($fmt));
    ($fmt:expr, $($arg:tt)*) => ({
        crate::__print(core::format_args!($fmt, $($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (crate::print!(core::concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (crate::print!(core::concat!($fmt, "\n"), $($arg)*));
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    exit(1)
}

pub fn io_write32(ptr: *mut u32, data: u32) {
    unsafe { ptr.write_volatile(data) }
}

pub fn io_read32(ptr: *const u32) -> u32 {
    unsafe { ptr.read_volatile() }
}

pub fn io_write64(ptr: *mut u64, data: u64) {
    unsafe { ptr.write_volatile(data) }
}

pub fn io_read64(ptr: *const u64) -> u64 {
    unsafe { ptr.read_volatile() }
}

pub fn num_cores() -> usize {
    extern "C" {
        static _num_cores: u8;
    }
    (unsafe { &_num_cores }) as *const u8 as usize
}

struct LockedHeapWithFlag(LockedHeap);

impl LockedHeapWithFlag {
    pub const fn empty() -> LockedHeapWithFlag {
        LockedHeapWithFlag(LockedHeap::new())
    }
}

impl Deref for LockedHeapWithFlag {
    type Target = LockedHeap;

    fn deref(&self) -> &LockedHeap {
        &self.0
    }
}

//when alloc/dealloc, disable the interrupt
unsafe impl GlobalAlloc for LockedHeapWithFlag {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let flag = save_flag();
        let ret = self.0.alloc(layout);
        restore_flag(flag);
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let flag = save_flag();
        self.0.dealloc(ptr, layout);
        restore_flag(flag);
    }
}

#[global_allocator]
static ALLOCATOR: LockedHeapWithFlag = LockedHeapWithFlag::empty();

#[alloc_error_handler]
fn oom(_layout: Layout) -> ! {
    crate::println!("oom!");
    crate::exit(2)
}

pub fn init_heap() {
    extern "C" {
        static mut _sheap: u8;
        static _heap_size: u8;
    }
    let m_sheap = unsafe { &mut _sheap } as *mut _ as usize;
    let m_heap_size = unsafe { &_heap_size } as *const u8 as usize;
    //interrupt should be enabled after calling init_heap()
    unsafe {
        ALLOCATOR.lock().add_to_heap(m_sheap, m_sheap + m_heap_size);
    }
}

#[no_mangle]
extern "C" fn malloc(size: usize, align: usize) -> usize {
    // let state = ALLOCATOR.lock().stats_alloc_actual();
    // println!("before malloc:{}",  state);
    unsafe { ALLOCATOR.alloc(Layout::from_size_align(size, align).unwrap()) as usize }
    // let state = ALLOCATOR.lock().stats_alloc_actual();
    // println!("after malloc:{}",  state);
}

#[no_mangle]
extern "C" fn free(ptr: usize, size: usize, align: usize) {
    // let state = ALLOCATOR.lock().stats_alloc_actual();
    // println!("before free:{}",  state);
    unsafe {
        ALLOCATOR.dealloc(
            ptr as *mut u8,
            Layout::from_size_align(size, align).unwrap(),
        );
    }
    // let state = ALLOCATOR.lock().stats_alloc_actual();
    // println!("after free:{}",  state);
}

#[export_name = "rsrt_start"]
fn rsrt_start() {
    use crate::hw_thread::idle;
    extern "C" {
        fn main() -> u32;
    }
    extern "C" {
        fn __boot_core_init();
    }

    let hartid = hartid();
    if hartid == 0 {
        init_heap();
        unsafe { __boot_core_init() };
        let ret = unsafe { main() };
        exit(ret);
    } else {
        idle()
    }
}
