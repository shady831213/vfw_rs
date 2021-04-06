use super::{Clk, GenClk};
use crate::spin::Mutex;
use core::ops::Deref;
struct ClkDivState {
    div: usize,
    mul: usize,
}

pub struct ClkDiv<T: Clk> {
    inner: T,
    state: Mutex<ClkDivState>,
}
impl<T: Clk> ClkDiv<T> {
    pub const fn new(clk: T, div: usize, mul: usize) -> ClkDiv<T> {
        ClkDiv {
            inner: clk,
            state: Mutex::new(ClkDivState { div, mul }),
        }
    }
}
impl<T: Clk> Clk for ClkDiv<T> {
    fn calculate(&self) -> usize {
        let state = self.state.lock();
        self.inner.calculate() * state.mul / state.div
    }
    fn enabled(&self) -> bool {
        self.inner.enabled()
    }
    fn enable(&self) {
        self.inner.enable();
    }
    fn disable(&self) {
        self.inner.disable();
    }
}

impl<T: Clk> GenClk for ClkDiv<T> {
    type Parent = T;
    fn parent(&self) -> Option<&Self::Parent> {
        Some(&self.inner)
    }
}

impl<T: Clk> Deref for ClkDiv<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
