// TODO: Move functionality of kernel/conf/runner.sh here
//       Then maybe we can compile on windows lol

use std::process::Command;
use std::path::PathBuf;

fn main() {
    println!("Hello, runner!");
    let efi_path = ovmf_prebuilt::ovmf_pure_efi();
    let efi_out_path = PathBuf::from("../target/efi/OVMF-pure-efi.fd");
    let mut efi_out_path_dir = efi_out_path.clone();
    efi_out_path_dir.pop();
    println!("Copy efi bios from `{}` to `{}`", efi_path.display(), efi_out_path.display());
    let _ = std::fs::create_dir_all(efi_out_path_dir);
    std::fs::copy(efi_path, efi_out_path).expect("Failed to copy file!");
    println!("Success! Calling `cargo run --release` on the kernel package now...");

    Command::new("cargo")
        .current_dir("../wasm_test")
        .arg("build")
        .arg("--release")
        .spawn().unwrap().wait().unwrap();

    Command::new("cargo")
        .current_dir("../kernel")
        .arg("run")
        .arg("--release")
        .spawn().unwrap().wait().unwrap();
}
