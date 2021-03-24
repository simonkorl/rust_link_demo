use std::{env, path::PathBuf};
extern crate dunce;

fn main() {
    // let library_name = "outer";
    // let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    // let library_dir = dunce::canonicalize(root.join("src")).unwrap();
    // println!("cargo:rustc-link-lib=dylib={}", library_name);
    // println!("cargo:rustc-link-search=native={}", env::join_paths(&[library_dir]).unwrap().to_str().unwrap());
    cc::Build::new()
        .file("src/link_c/inner.cxx")
        .include("src/link_c/include/")
        .cpp(true)
        .compile("inner");
}