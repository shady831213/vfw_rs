use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rerun-if-changed=include");
    println!("features:{:?}", env::var("CARGO_FEATURE_MAILBOX_RS"));
    println!("target:{:?}", env::var("TARGET"));
    if env::var("CARGO_FEATURE_MAILBOX_RS").is_ok() {
        fs::File::create(out_dir.join("mailbox.x"))
            .unwrap()
            .write_all(include_bytes!("src/hal/mailbox/mailbox.x"))
            .unwrap();
    }
    println!("cargo:rerun-if-changed=src/hal/mailbox/mailbox.x");
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=build.rs");
}
