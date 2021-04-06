use super::{Clk, GenClk};
#[derive(Debug)]
#[repr(usize)]
pub enum ClkMuxError {
    InvalidId,
}
pub type ClkMuxResult<T> = Result<T, ClkMuxError>;
pub trait ClkMux: GenClk {
    fn switch_to(&self, id: usize) -> ClkMuxResult<Option<&Self::Parent>>;
    fn switch(&self, id: usize) -> ClkMuxResult<()> {
        let prev_p = self.parent();
        self.switch_to(id)?;
        if let Some(p) = prev_p {
            p.disable();
        }
        Ok(())
    }
}
