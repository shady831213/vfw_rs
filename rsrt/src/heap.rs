#[cfg(not(feature = "max_cores_1"))]
mod lock_heap {
    use alloc::alloc::GlobalAlloc;
    use buddy_system_allocator::LockedHeap;
    use core::alloc::Layout;
    use core::ops::Deref;
    use crate::{restore_flag, save_flag};
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

    #[link_section = ".synced.bss"]
    #[global_allocator]
    static ALLOCATOR: LockedHeapWithFlag = LockedHeapWithFlag::empty();

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
}

#[cfg(feature = "max_cores_1")]
mod unlock_heap {
    use alloc::alloc::GlobalAlloc;
    use buddy_system_allocator::Heap;
    use core::alloc::Layout;
    use core::ptr::NonNull;
    use crate::{restore_flag, save_flag};

    #[link_section = ".synced.bss"]
    static mut H:Heap = Heap::empty();
    struct HeapWithFlag;

    unsafe impl GlobalAlloc for HeapWithFlag {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let flag = save_flag();
            let ret = H.alloc(layout)
                .ok()
                .map_or(0 as *mut u8, |allocation| allocation.as_ptr());
            restore_flag(flag);
            ret
        }
    
        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            let flag = save_flag();
            H.dealloc(NonNull::new_unchecked(ptr), layout);
            restore_flag(flag);
        }
    }

    #[link_section = ".synced.bss"]
    #[global_allocator]
    static ALLOCATOR: HeapWithFlag = HeapWithFlag;

    pub fn init_heap() {
        extern "C" {
            static mut _sheap: u8;
            static _heap_size: u8;
        }
        let m_sheap = unsafe { &mut _sheap } as *mut _ as usize;
        let m_heap_size = unsafe { &_heap_size } as *const u8 as usize;
        //interrupt should be enabled after calling init_heap()
        unsafe {
            H.add_to_heap(m_sheap, m_sheap + m_heap_size);
        }
    }

    #[no_mangle]
    extern "C" fn malloc(size: usize, align: usize) -> usize {
        unsafe { ALLOCATOR.alloc(Layout::from_size_align(size, align).unwrap()) as usize }
    }

    #[no_mangle]
    extern "C" fn free(ptr: usize, size: usize, align: usize) {
        unsafe {
            ALLOCATOR.dealloc(
                ptr as *mut u8,
                Layout::from_size_align(size, align).unwrap(),
            );
        }
    }
}

#[cfg(not(feature = "max_cores_1"))]
pub use lock_heap::*;
#[cfg(feature = "max_cores_1")]
pub use unlock_heap::*;