// NB: this file is not present in the published version of this crate.

use std::path::PathBuf;

fn main() {
    if !cfg!(feature = "locally-developed") {
        return;
    }

    let mut out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

    while !out_dir.ends_with("debug") && !out_dir.ends_with("release") {
        assert!(out_dir.pop(), "reached the root dir");
    }

    let loc = out_dir
        .join("cargo-wasi")
        .with_extension(std::env::consts::EXE_EXTENSION);

    if !loc.exists() {
        eprintln!("the `cargo-wasi` binary needs to be built before this crate");
        eprintln!("{:?} does not exist", loc);
        std::process::exit(1);
    }

    println!("cargo:rustc-env=BYTES_LOC={}", loc.to_str().unwrap());
}
