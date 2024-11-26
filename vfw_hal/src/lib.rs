#![no_std]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]
#![feature(linkage)]
pub extern crate embedded_hal;
pub extern crate nb;
pub mod clk;
pub mod ddr;
pub mod delay;
pub mod io;
pub mod uart;
use vfw_primitives::*;
