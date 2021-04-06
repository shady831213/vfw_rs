use super::Clk;
pub trait Pll: Clk {
    fn is_locked(&self) -> bool;
    fn wait_lock(&self) {
        while !self.is_locked() {}
    }
}
