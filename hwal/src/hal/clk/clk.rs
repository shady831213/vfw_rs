pub trait Clk {
    fn calculate(&self) -> usize;
    fn enable(&self);
    fn enabled(&self) -> bool;
    fn disable(&self);
}

pub trait GenClk: Clk {
    type Parent: Clk;
    fn parent(&self) -> Option<&Self::Parent>;
}
