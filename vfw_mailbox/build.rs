use vfw_build_utils::{HeaderDir, LinkFile};

fn main() {
    LinkFile::new("mailbox.x")
        .unwrap()
        .add_file("src/link.x")
        .unwrap();
    HeaderDir::new().unwrap().add_dir("include").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
