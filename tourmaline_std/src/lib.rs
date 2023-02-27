// Expose the raw ABI
pub mod abi;

pub fn kernel_log<S: Into<String>>(s: S) {
    let s = s.into();
    abi::abi_kernel_log(s);
}
