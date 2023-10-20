use crate::arch::{default_trap_handler, dummy_trap_handler, FlowContext};
use core::{marker::PhantomPinned, ops::Range, ptr::NonNull};
// modified from https://github.com/YdrMaster/fast-trap
// The MIT License (MIT)
// Copyright © 2022 YdrMaster
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
#[repr(C)]
pub(crate) struct TrapContext {
    pub(crate) context: NonNull<FlowContext>,
    pub(crate) handler: TrapHandler,
    pub(crate) scratch: usize,
    pub(crate) range: Range<usize>,
    pub(crate) drop: fn(Range<usize>),
    pinned: PhantomPinned,
}

pub type TrapHandler = extern "C" fn(ctx: &mut FlowContext);

pub union TrapVector {
    pub handler: TrapHandler,
    pub reserved: usize,
}

impl TrapVector {
    #[inline]
    pub unsafe fn handle(&self, ctx: &mut FlowContext) {
        if self.reserved == 0 {
            default_trap_handler(ctx);
        } else {
            (self.handler)(ctx);
        }
    }

    #[inline]
    pub unsafe fn handle_or_dummy(&self, ctx: &mut FlowContext) {
        if self.reserved == 0 {
            dummy_trap_handler();
        } else {
            (self.handler)(ctx);
        }
    }
}
