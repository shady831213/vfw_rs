use build_tools::*;
use std::env;
use std::path::PathBuf;
fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tests_build(&out_dir, &env::var("RISCV_TOOLCHAIN_PREFIX").unwrap_or("".to_string()));
    println!("cargo:rerun-if-changed=build.rs");
}
