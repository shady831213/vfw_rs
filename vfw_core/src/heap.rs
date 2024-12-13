#[cfg(any(
    feature = "max_cores_128",
    feature = "max_cores_64",
    feature = "max_cores_32",
    feature = "max_cores_16",
    feature = "max_cores_8",
    feature = "max_cores_4",
    feature = "max_cores_2"
))]
mod lock_heap {
    use crate::{restore_flag, save_flag};
    use alloc::alloc::GlobalAlloc;
    use buddy_system_allocator::LockedHeap;
    use core::alloc::Layout;
    use core::ops::Deref;
    struct LockedHeapWithFlag<const N: usize>(LockedHeap<N>);

    impl<const N: usize> LockedHeapWithFlag<N> {
        pub const fn empty() -> Self {
            LockedHeapWithFlag(LockedHeap::new())
        }
    }

    impl<const N: usize> Deref for LockedHeapWithFlag<N> {
        type Target = LockedHeap<N>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    //when alloc/dealloc, disable the interrupt
    unsafe impl<const N: usize> GlobalAlloc for LockedHeapWithFlag<N> {
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
    static ALLOCATOR: LockedHeapWithFlag<32> = LockedHeapWithFlag::empty();

    pub fn init_heap() {
        extern "C" {
            static mut _sheap: u8;
            static _heap_size: usize;
        }
        let m_sheap = &raw mut _sheap as *mut _ as usize;
        let m_heap_size = &raw const _heap_size as *const usize as usize;
        //interrupt should be enabled after calling init_heap()
        unsafe {
            ALLOCATOR.lock().add_to_heap(m_sheap, m_sheap + m_heap_size);
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
mod unlock_heap {
    use crate::{restore_flag, save_flag};
    use alloc::alloc::GlobalAlloc;
    use buddy_system_allocator::Heap;
    use core::alloc::Layout;
    use core::ptr::NonNull;

    #[link_section = ".synced.bss"]
    static mut H: Heap<32> = Heap::empty();
    struct HeapWithFlag;

    unsafe impl GlobalAlloc for HeapWithFlag {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let flag = save_flag();
            let ret = H
                .alloc(layout)
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
            static _heap_size: usize;
        }
        let m_sheap = unsafe { &mut _sheap } as *mut _ as usize;
        let m_heap_size = unsafe { &_heap_size } as *const usize as usize;
        //interrupt should be enabled after calling init_heap()
        unsafe {
            H.add_to_heap(m_sheap, m_sheap + m_heap_size);
        }
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
pub use lock_heap::*;
#[cfg(not(any(
    feature = "max_cores_128",
    feature = "max_cores_64",
    feature = "max_cores_32",
    feature = "max_cores_16",
    feature = "max_cores_8",
    feature = "max_cores_4",
    feature = "max_cores_2"
)))]
pub use unlock_heap::*;
