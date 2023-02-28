// Expose the raw ABI
pub mod abi;

pub fn kernel_log<S: Into<String>>(s: S) {
    let s = s.into();
    abi::abi_sys_log(s);
}

pub fn _print(args: std::fmt::Arguments) {
    kernel_log(format!("{}", args));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}
