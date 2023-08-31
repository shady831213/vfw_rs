use vfw_build_utils::HeaderDir;

fn main() {
    HeaderDir::new()
        .unwrap()
        .add_dep("vfw_primitives")
        .unwrap()
        .add_dep("vfw_utils")
        .unwrap()
        .add_dep("vfw_core")
        .unwrap()
        .add_dep("vfw_mailbox")
        .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
