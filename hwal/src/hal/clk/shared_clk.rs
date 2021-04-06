use super::Clk;
use crate::spin::Mutex;
use core::ops::Deref;

struct SharedClkState {
    refs: isize,
}

pub struct SharedClk<T: Clk> {
    inner: T,
    state: Mutex<SharedClkState>,
}
impl<T: Clk> SharedClk<T> {
    pub const fn new(clk: T) -> SharedClk<T> {
        SharedClk {
            inner: clk,
            state: Mutex::new(SharedClkState { refs: 0 }),
        }
    }
}
impl<T: Clk> Clk for SharedClk<T> {
    fn calculate(&self) -> usize {
        self.inner.calculate()
    }
    fn enable(&self) {
        let mut state = self.state.lock();
        if state.refs == 0 {
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
        if state.refs <= 0 {
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
