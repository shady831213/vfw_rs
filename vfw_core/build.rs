use serde::Serialize;
use std::env;
use vfw_build_utils::{HeaderDir, LinkFile};

#[derive(Serialize)]
struct Stack {
    stack_size: String,
}
fn main() {
    LinkFile::new("vfw.x")
        .unwrap()
        .render_file("src/link.x", || Stack {
            stack_size: if env::var("CARGO_FEATURE_STACK_NON_PRIV").is_ok() {
                "(_stack_size - _provide_base) * (_num_cores - _provide_base)"
            } else {
                "_stack_size - _provide_base"
            }
            .to_string(),
        })
        .unwrap();
    HeaderDir::new().unwrap().add_dir("include").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
