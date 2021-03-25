use build_tools::*;
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::File::create(out_dir.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rustc-link-search={}", out_dir.display());
    platform_build();
    println!("cargo:rerun-if-changed=build.rs");
}
