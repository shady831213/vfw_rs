mod bitmap;
pub use bitmap::*;
mod allocator;
pub use allocator::*;

pub struct ConstConstraint<const B: bool>;
pub trait CCTrue {}
impl CCTrue for ConstConstraint<true> {}
