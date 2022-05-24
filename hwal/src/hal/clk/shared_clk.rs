use super::Clk;
#[cfg(not(feature = "max_cores_1"))]
use spin::Mutex;
#[cfg(feature = "max_cores_1")]
use core::cell::RefCell;
use core::ops::Deref;

struct SharedClkState {
    refs: isize,
}

pub struct SharedClk<T: Clk> {
    inner: T,
    #[cfg(not(feature = "max_cores_1"))]
    state: Mutex<SharedClkState>,
    #[cfg(feature = "max_cores_1")]
    state: RefCell<SharedClkState>
}
#[cfg(feature = "max_cores_1")]
unsafe impl <T:Clk> Sync for SharedClk<T> {}

impl<T: Clk> SharedClk<T> {
    pub const fn new(clk: T) -> SharedClk<T> {
        SharedClk {
            inner: clk,
            #[cfg(not(feature = "max_cores_1"))]
            state: Mutex::new(SharedClkState { refs: 0 }),
            #[cfg(feature = "max_cores_1")]
            state: RefCell::new(SharedClkState { refs: 0 }),
        }
    }
}
impl<T: Clk> Clk for SharedClk<T> {
    fn calculate(&self) -> usize {
        self.inner.calculate()
    }
    fn enable(&self) {
        #[cfg(not(feature = "max_cores_1"))]
        let mut state = self.state.lock();
        #[cfg(feature = "max_cores_1")]
        let mut state = self.state.borrow_mut();
        if state.refs == 0 && !self.inner.enabled(){
            self.inner.enable();
        }
        state.refs += 1;
    }
    fn enabled(&self) -> bool {
        #[cfg(not(feature = "max_cores_1"))]
        let state = self.state.lock();
        #[cfg(feature = "max_cores_1")]
        let state = self.state.borrow();
        state.refs > 0 && self.inner.enabled()
    }
    fn disable(&self) {
        #[cfg(not(feature = "max_cores_1"))]
        let mut state = self.state.lock();
        #[cfg(feature = "max_cores_1")]
        let mut state = self.state.borrow_mut();
        state.refs -= 1;
        if state.refs <= 0 && self.inner.enabled(){
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
