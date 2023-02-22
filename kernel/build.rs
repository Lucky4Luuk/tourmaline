fn main() {
    let shell = std::path::PathBuf::from(std::env::var_os("CARGO_BIN_FILE_SHELL_shell").unwrap());
    println!("cargo:rustc-env=SHELL_PATH={}", shell.display());
}
