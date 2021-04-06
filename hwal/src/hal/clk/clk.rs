pub trait Clk {
    fn calculate(&self) -> usize;
    fn enable(&self);
    fn disable(&self);
}

pub trait GenClk {
    type Parent: Clk;
    fn parent(&self) -> Option<&Self::Parent>;
}