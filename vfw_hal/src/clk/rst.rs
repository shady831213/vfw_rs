pub trait SoftReset {
    fn set_rst(&self);
    fn clr_rst(&self);
}