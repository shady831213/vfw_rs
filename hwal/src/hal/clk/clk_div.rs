use super::{Clk, GenClk};
use crate::spin::Mutex;
use core::ops::Deref;
struct ClkDivState {
    div: usize,
    mul: usize,
}

pub struct ClkDiv<'a, T: Clk> {
    inner: &'a T,
    state: Mutex<ClkDivState>,
}
impl<'a, T: Clk> ClkDiv<'a, T> {
    pub const fn new(clk: &'a T, div: usize, mul: usize) -> ClkDiv<'a, T> {
        ClkDiv {
            inner: clk,
            state: Mutex::new(ClkDivState { div, mul }),
        }
    }
}
impl<'a, T: Clk> Clk for ClkDiv<'a, T> {
    fn calculate(&self) -> usize {
        let state = self.state.lock();
        self.inner.calculate() * state.mul / state.div
    }
    fn enable(&self) {
        self.inner.enable();
    }
    fn disable(&self) {
        self.inner.disable();
    }
}

impl<'a, T: Clk> GenClk for ClkDiv<'a, T> {
    type Parent = T;
    fn parent(&self) -> Option<&Self::Parent> {
        Some(self.inner)
    }
}

impl<'a, T: Clk> Deref for ClkDiv<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
