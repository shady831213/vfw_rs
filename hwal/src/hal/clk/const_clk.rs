use super::Clk;
pub struct ConstClk {
    freq: usize,
}
impl ConstClk {
    pub const fn new(freq: usize) -> ConstClk {
        ConstClk { freq }
    }
}
impl Clk for ConstClk {
    fn calculate(&self) -> usize {
        self.freq
    }
    fn enable(&self) {}
    fn enabled(&self) -> bool {
        true
    }
    fn disable(&self) {}
}
