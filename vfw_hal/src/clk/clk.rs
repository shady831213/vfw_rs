pub trait Clk {
    fn calculate(&self) -> usize;
    fn enable(&self) {}
    fn enabled(&self) -> bool {
        true
    }
    fn disable(&self) {}
}

pub trait GenClk: Clk {
    type Parent: Clk;
    fn parent(&self) -> Option<&Self::Parent>;
}

impl<T: Clk> Clk for &T {
    fn calculate(&self) -> usize {
        (*self).calculate()
    }
    fn enable(&self) {
        (*self).enable()
    }
    fn enabled(&self) -> bool {
        (*self).enabled()
    }
    fn disable(&self) {
        (*self).disable()
    }
}
