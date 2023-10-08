mod exception;
pub use exception::*;
mod interrupt;
pub use interrupt::*;
pub(super) mod trap;
use riscv::register::{mcause, mepc, mtval};
fn default_trap_handler() {
    panic!(
        "Unexpected trap: cause:{:x?}, mepc:{:x}, mtval:{:x}",
        mcause::read(),
        mepc::read(),
        mtval::read()
    );
}

fn dummy_trap_handler() {}
