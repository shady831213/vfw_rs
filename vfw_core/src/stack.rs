use core::ptr::NonNull;
use fast_trap::{FastHandler, FlowContext, FreeTrapStack};
extern "C" {
    static mut _sstack: u8;
    static _stack_size: usize;
    static _provide_base: usize;
}

pub trait Stack {
    fn start(&self) -> usize;
    fn size(&self) -> usize;
    #[inline(always)]
    fn end(&self) -> usize {
        self.start() - self.size()
    }
    #[inline(always)]
    fn load_context(&self, context_ptr: NonNull<FlowContext>, fast_handler: FastHandler) {
        core::mem::forget(
            FreeTrapStack::new(self.end()..self.start(), |_| {}, context_ptr, fast_handler)
                .unwrap()
                .load(),
        );
    }
}

pub struct VfwStack;
impl Stack for VfwStack {
    #[inline(always)]
    fn start(&self) -> usize {
        stack_start()
    }
    #[inline(always)]
    fn size(&self) -> usize {
        stack_size()
    }
}

#[no_mangle]
#[inline(always)]
pub extern "C" fn stack_start() -> usize {
    let m_sstack = unsafe { &mut _sstack } as *mut _ as usize;
    #[cfg(feature = "stack_non_priv")]
    {
        m_sstack - stack_size() * crate::hartid()
    }
    #[cfg(not(feature = "stack_non_priv"))]
    {
        m_sstack
    }
}

#[no_mangle]
#[inline(always)]
pub extern "C" fn stack_size() -> usize {
    let m_stack_size = unsafe { &_stack_size } as *const usize as usize;
    let m_provide_base = unsafe { &_provide_base } as *const usize as usize;
    m_stack_size - m_provide_base
}
