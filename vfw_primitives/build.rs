use vfw_build_utils::HeaderDir;

fn main() {
    HeaderDir::new().unwrap().add_dir("include").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
