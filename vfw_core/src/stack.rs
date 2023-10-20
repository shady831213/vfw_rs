use crate::arch::arch;
use crate::trap::{TrapContext, TrapHandler};
use core::{
    alloc::Layout,
    marker::PhantomPinned,
    mem::{align_of, forget, MaybeUninit},
    ops::Range,
    ptr::NonNull,
};
use fast_trap::FlowContext;

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
    fn load_context(&self, context_ptr: NonNull<FlowContext>, handler: TrapHandler) {
        core::mem::forget(
            FreeTrapStack::new(self.end()..self.start(), |_| {}, context_ptr, handler)
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

// modified from https://github.com/YdrMaster/fast-trap
// The MIT License (MIT)
// Copyright © 2022 YdrMaster
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
pub struct FreeTrapStack(NonNull<TrapContext>);
pub struct LoadedTrapStack(usize);
#[derive(Debug)]
pub struct IllegalStack;
impl FreeTrapStack {
    pub fn new(
        range: Range<usize>,
        drop: fn(Range<usize>),
        context_ptr: NonNull<FlowContext>,
        handler: TrapHandler,
    ) -> Result<Self, IllegalStack> {
        const LAYOUT: Layout = Layout::new::<TrapContext>();
        let bottom = range.start;
        let top = range.end;
        let ptr = (top - LAYOUT.size()) & !(LAYOUT.align() - 1);
        if ptr >= bottom {
            let ctx = unsafe { &mut *(ptr as *mut TrapContext) };
            ctx.range = range;
            ctx.drop = drop;
            ctx.context = context_ptr;
            ctx.handler = handler;
            Ok(Self(unsafe { NonNull::new_unchecked(ctx) }))
        } else {
            Err(IllegalStack)
        }
    }

    #[inline]
    pub fn load(self) -> LoadedTrapStack {
        let scratch = arch::exchange_scratch(self.0.as_ptr() as _);
        forget(self);
        LoadedTrapStack(scratch)
    }
}

impl Drop for FreeTrapStack {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let handler = self.0.as_ref();
            (handler.drop)(handler.range.clone());
        }
    }
}

impl LoadedTrapStack {
    #[inline]
    pub const fn val(&self) -> usize {
        self.0
    }

    #[inline]
    pub fn unload(self) -> FreeTrapStack {
        let ans = unsafe { self.unload_unchecked() };
        forget(self);
        ans
    }

    #[inline]
    unsafe fn unload_unchecked(&self) -> FreeTrapStack {
        let ptr = arch::exchange_scratch(self.0) as *mut TrapContext;
        let handler = unsafe { NonNull::new_unchecked(ptr) };
        FreeTrapStack(handler)
    }
}

impl Drop for LoadedTrapStack {
    #[inline]
    fn drop(&mut self) {
        drop(unsafe { self.unload_unchecked() })
    }
}
