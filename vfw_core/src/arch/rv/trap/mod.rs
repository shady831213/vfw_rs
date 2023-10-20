mod exception;
pub use exception::*;
mod interrupt;
pub use interrupt::*;
pub(super) mod trap;
use riscv::register::{mcause, mepc, mtval};
pub use trap::FlowContext;
fn default_trap_handler() {
    panic!(
        "Unexpected trap: cause:{:?}, mepc:{:x}, mtval:{:x}",
        mcause::read().cause(),
        mepc::read(),
        mtval::read()
    );
}

fn dummy_trap_handler() {}
