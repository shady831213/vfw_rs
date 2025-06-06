use serde::Serialize;
use std::env;
use vfw_build_utils::{HeaderDir, LinkFile};

#[derive(Serialize)]
struct Link {
    load_bss: String,
    stack_size: String,
}
fn main() {
    LinkFile::new("vfw.x")
        .unwrap()
        .render_file("src/link.x", || Link {
            load_bss: if env::var("CARGO_FEATURE_LOAD_BSS").is_ok() {
                ""
            } else {
                r"(NOLOAD)"
            }
            .to_string(),
            stack_size: if env::var("CARGO_FEATURE_STACK_NON_PRIV").is_ok() {
                "(_stack_size) * (_num_cores)"
            } else {
                "_stack_size"
            }
            .to_string(),
        })
        .unwrap();
    HeaderDir::new().unwrap().add_dir("include").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
