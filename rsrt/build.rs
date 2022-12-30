use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut file = fs::File::create(out_dir.join("linker.x")).unwrap();
    if env::var("CARGO_FEATURE_STACK_NON_PRIV").is_ok() {
        file.write_all(
            b"
SECTIONS
{    
        .stack (NOLOAD) : ALIGN(1K) {
            _estack = .;
            . += _stack_size * _num_cores;
            _sstack = .;
        } > REGION_STACK
}
        ",
        )
    } else {
        file.write_all(
            b"
SECTIONS
{    
        .stack (NOLOAD) : ALIGN(1K) {
            _estack = .;
            . += _stack_size;
            _sstack = .;
        } > REGION_STACK
}
        ",
        )
    }
    .unwrap();
    file.write_all(include_bytes!("linker.x")).unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=linker.x");
}
