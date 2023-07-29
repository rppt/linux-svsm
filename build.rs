extern crate bindgen;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const LIBCRT_SRC: &str = "external/libcrt";
const LIBM_SRC: &str = "external/libm";
const WOLFSSL_SRC: &str = "external/wolfssl";
const MSTPM_SRC: &str = "external/ms-tpm-20-ref";
const BUILD_DIR: &str = "external/build";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=libtpm.h");

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let wolfssl_src = format!("{}/{}", manifest_dir, WOLFSSL_SRC);
    let mstpm_src = Path::new(&format!("{}/{}", manifest_dir, MSTPM_SRC)).join("TPMCmd");
    let build_dir = format!("{}/{}", manifest_dir, BUILD_DIR);

    println!("{}", manifest_dir);

    fs::create_dir_all(&build_dir).expect("Failed to create build directory");
    let build_lib = Path::new(&build_dir).join("lib");
    fs::create_dir_all(&build_lib).expect("Failed to create build directory");

    let bindings = bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("cty")
        .clang_arg("-DHASH_LIB=Wolf")
        .clang_arg("-DSYM_LIB=Wolf")
        .clang_arg("-DMATH_LIB=Wolf")
        .clang_arg(format!(
            "-I{}",
            Path::new(&wolfssl_src).join("amd-svsm").to_str().unwrap()
        ))
        .clang_arg(format!(
            "-I{}",
            Path::new(&build_dir).join("include").to_str().unwrap()
        ))
        .clang_arg(format!(
            "-I{}",
            mstpm_src.join("tpm").join("include").to_str().unwrap()
        ))
        .clang_arg(format!(
            "-I{}",
            mstpm_src
                .join("tpm")
                .join("include")
                .join("prototypes")
                .to_str()
                .unwrap()
        ))
        .clang_arg(format!(
            "-I{}",
            mstpm_src.join("Platform").join("include").to_str().unwrap()
        ))
        .clang_arg(format!(
            "-I{}",
            mstpm_src
                .join("Platform")
                .join("include")
                .join("prototypes")
                .to_str()
                .unwrap()
        ))
        .header("libtpm.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
