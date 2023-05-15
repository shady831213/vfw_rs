use super::Clk;
use core::cell::RefCell;
use core::ops::Deref;
use spin::Mutex;

struct SharedClkState {
    refs: isize,
}

pub struct LockedSharedClk<T: Clk> {
    inner: T,
    state: Mutex<SharedClkState>,
}
unsafe impl<T: Clk> Sync for LockedSharedClk<T> {}

impl<T: Clk> LockedSharedClk<T> {
    pub fn new(clk: T) -> LockedSharedClk<T> {
        LockedSharedClk {
            inner: clk,
            state: Mutex::new(SharedClkState { refs: 0 }),
        }
    }
}
impl<T: Clk> Clk for LockedSharedClk<T> {
    fn calculate(&self) -> usize {
        self.inner.calculate()
    }
    fn enable(&self) {
        let mut state = self.state.lock();
        if state.refs == 0 && !self.inner.enabled() {
            self.inner.enable();
        }
        state.refs += 1;
    }
    fn enabled(&self) -> bool {
        let state = self.state.lock();
        state.refs > 0 && self.inner.enabled()
    }
    fn disable(&self) {
        let mut state = self.state.lock();
        state.refs -= 1;
        if state.refs <= 0 && self.inner.enabled() {
            self.inner.disable();
            state.refs = 0;
        }
    }
}

impl<T: Clk> Deref for LockedSharedClk<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct SharedClk<T: Clk> {
    inner: T,
    state: RefCell<SharedClkState>,
}
impl<T: Clk> SharedClk<T> {
    pub const fn new(clk: T) -> SharedClk<T> {
        SharedClk {
            inner: clk,
            state: RefCell::new(SharedClkState { refs: 0 }),
        }
    }
}
impl<T: Clk> Clk for SharedClk<T> {
    fn calculate(&self) -> usize {
        self.inner.calculate()
    }
    fn enable(&self) {
        let mut state = self.state.borrow_mut();
        if state.refs == 0 && !self.inner.enabled() {
            self.inner.enable();
        }
        state.refs += 1;
    }
    fn enabled(&self) -> bool {
        let state = self.state.borrow();
        state.refs > 0 && self.inner.enabled()
    }
    fn disable(&self) {
        let mut state = self.state.borrow_mut();
        state.refs -= 1;
        if state.refs <= 0 && self.inner.enabled() {
            self.inner.disable();
            state.refs = 0;
        }
    }
}

impl<T: Clk> Deref for SharedClk<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
