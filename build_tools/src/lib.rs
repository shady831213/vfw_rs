use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
pub extern crate bindgen;
pub extern crate cc;

pub fn walk_dir<F: FnMut(PathBuf) -> Result<(), String>>(
    dir: &Path,
    f: &mut F,
) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.is_dir() {
            walk_dir(&path, f)?
        }
        f(path)?;
    }
    Ok(())
}

pub fn copy_dir(src:&Path, dest:&Path) ->  Result<(), String> {
    fs::create_dir_all(&dest).map_err(|e|{format!("{:?}:{:?}", dest.display(), e)})?;
    walk_dir(&src, &mut |p:PathBuf|{
        let dest_p = dest.join(p.strip_prefix(&src).unwrap());
        if p.is_dir() {
            fs::create_dir_all(&dest_p).map_err(|e|{format!("{:?}:{:?}", dest_p.display(), e)})?;
        } else {
            println!("cargo:rerun-if-changed={}", p.display());
            fs::copy(&p, &dest_p).map_err(|e|{format!("Copy {:?} to {:?}:{:?}", p.display(), dest_p.display(), e)})?;
        }
        Ok(())  
    })
}

pub fn link_dir(src:&Path, dest:&Path) ->  Result<(), String> {
    fs::create_dir_all(&dest).map_err(|e|{format!("{:?}:{:?}", dest.display(), e)})?;
    walk_dir(&src, &mut |p:PathBuf|{
        let dest_p = dest.join(p.strip_prefix(&src).unwrap());
        if p.is_dir() {
            fs::create_dir_all(&dest_p).map_err(|e|{format!("{:?}:{:?}", dest_p.display(), e)})?;
        } else {
            use std::os::unix::fs::symlink;
            println!("cargo:rerun-if-changed={}", p.display());
            if dest_p.exists() {
                fs::remove_file(&dest_p).map_err(|e|{format!("Link {:?} to {:?}:{:?}", p.display(), dest_p.display(), e)})?;
            }
            symlink(&p, &dest_p).map_err(|e|{format!("Link {:?} to {:?}:{:?}", p.display(), dest_p.display(), e)})?;
        }
        Ok(())  
    })
}

pub fn c_src_dir(root_dir: &Path) -> Result<(Vec<PathBuf>, Vec<PathBuf>), String> {
    let mut c_files = vec![];
    let mut incdir = vec![];
    incdir.push(PathBuf::from(root_dir));
    println!("cargo:rerun-if-changed={}", root_dir.display());
    walk_dir(root_dir, &mut |p: PathBuf| {
        if p.is_file() {
            if let Some(ext) = p.extension() {
                if ext == OsStr::new("c") {
                    println!("cargo:rerun-if-changed={}", p.display());
                    c_files.push(p);
                } else if ext == OsStr::new("h") {
                    println!("cargo:rerun-if-changed={}", p.display());
                }
            }
        } else if p.is_dir() {
            println!("cargo:rerun-if-changed={}", p.display());
            incdir.push(p);
        }
        Ok(())
    })?;
    Ok((c_files, incdir))
}

pub fn build_c_files<'a>(
    root_dir: &Path,
    build: &'a mut cc::Build,
) -> Result<Option<&'a mut cc::Build>, String> {
    let (c_files, incdir) = c_src_dir(root_dir)?;
    if c_files.len() > 0 {
        println!("cargo:rerun-if-changed={}", root_dir.display());
        println!("c_fiels:{:?}", c_files);
        println!("incdir:{:?}", incdir);
        let hwal_inc_dir = PathBuf::from("../../../hwal/include");
        c_src_dir(&hwal_inc_dir).unwrap();
        println!("cargo:rerun-if-changed={}", hwal_inc_dir.display());
        let rsrt_inc_dir = PathBuf::from("../../../rsrt/include");
        c_src_dir(&rsrt_inc_dir).unwrap();
        println!("cargo:rerun-if-changed={}", rsrt_inc_dir.display());
        Ok(Some(
            build
                .files(&c_files)
                .includes(&incdir)
                .include(&root_dir)
                .include(&hwal_inc_dir)
                .include(&rsrt_inc_dir),
        ))
    } else {
        Ok(None)
    }
}

pub fn tests_build(out_dir: &Path, toolchain_prefix: &str) {
    tests_build_with(out_dir, toolchain_prefix, |b| {b})
}

pub fn tests_build_with<F: FnMut(&mut cc::Build)->&mut cc::Build>(
    out_dir: &Path,
    toolchain_prefix: &str,
    mut f: F,
) {
    println!("cargo:rerun-if-env-changed=TESTNAME");
    let test_name = env::var("TESTNAME").unwrap();
    let test_dir = Path::new("src/bin").join(&test_name);
    let mut c_build = cc::Build::new();
    if let Some(build) = build_c_files(&test_dir, &mut c_build).unwrap() {
        let b = build
            .compiler(format!("{}gcc", toolchain_prefix))
            .archiver(format!("{}ar", toolchain_prefix))
            .out_dir(out_dir)
            .flag("-Wno-main")
            .flag("-Wno-strict-aliasing")
            .flag("-Wno-builtin-declaration-mismatch");
        f(b).compile(&test_name);
    }
}

pub fn bindgen_build() {
    let incdir = PathBuf::from("include");
    println!("cargo:rerun-if-changed={}", incdir.display());
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", incdir.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .detect_include_paths(true)
        .ignore_functions()
        .ignore_methods()
        .clang_arg(format!(
            "--target={}",
            env::var("CARGO_CFG_TARGET_ARCH").unwrap()
        ))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("rsrt_bindings.rs"))
        .expect("Couldn't write bindings!");
}
