#![feature(exit_status_error)]
use std::process::Command;

fn main() {
    let wasm_test = std::path::PathBuf::from(std::env::var_os("CARGO_CDYLIB_FILE_WASM_TEST_wasm_test").unwrap());
    println!("cargo:rustc-env=WASM_TEST_PATH={}", wasm_test.display());
}
