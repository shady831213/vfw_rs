mod bitmap;
pub use bitmap::*;
#[cfg(feature = "alloc")]
mod allocator;
#[cfg(feature = "alloc")]
pub use allocator::*;

pub struct ConstConstraint<const B: bool>;
pub trait CCTrue {}
impl CCTrue for ConstConstraint<true> {}
