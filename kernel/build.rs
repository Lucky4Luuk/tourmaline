use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Get the name of the package.
    let kernel_name = env::var("CARGO_PKG_NAME")?;

    // Tell rustc to pass the linker script to the linker.
    println!("cargo:rustc-link-arg-bin={kernel_name}=-Tkernel/conf/linker.ld");

    // Have cargo rerun this script if the linker script or CARGO_PKG_ENV changes
    println!("cargo:rerun-if-changed=kernel/conf/linker.ld");
    println!("cargo:rerun-if-env-changed=CARG_PKG_NAME");

    Ok(())
}
