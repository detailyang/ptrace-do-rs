extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    // x86_64
    // linux
    // let target = env::var("TARGET").expect("TARGET was not set");
    // let host = env::var("HOST").expect("HOST was not set");
    // let num_jobs = env::var("NUM_JOBS").expect("NUM_JOBS was not set");
    // let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR was not set"));
    // let src_dir = env::current_dir().expect("failed to get current directory");
    //
    // println!("TARGET={}", target.clone());
    // println!("HOST={}", host.clone());
    // println!("NUM_JOBS={}", num_jobs.clone());
    // println!("OUT_DIR={:?}", out_dir);
    // let build_dir = out_dir.join("build");
    // println!("BUILD_DIR={:?}", build_dir);
    // println!("SRC_DIR={:?}", src_dir);

    cc::Build::new()
        .file("src/ptrace_do/libptrace_do.c")
        .file("src/ptrace_do/parse_maps.c")
        .include("src/ptrace_do")
        .compile("libptrace");

    println!("cargo:rustc-link-lib=static=libptrace");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg("-Isrc/ptrace_do")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
