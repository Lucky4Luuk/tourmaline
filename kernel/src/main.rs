#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use] extern crate log;

use bootloader_api::{
    entry_point,
    BootInfo,
    config::{
        BootloaderConfig,
        Mapping
    },
};

mod util;
mod panic_handler;
mod framebuffer;
mod logger;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    framebuffer::init(boot_info);
    framebuffer::fb_mut().set_clear_color([32,32,32]);
    framebuffer::fb_mut().clear();
    logger::init(log::LevelFilter::max()).unwrap();
    info!("Hello kernel!");
    // error!("SOMETHING WENT WRONG OH MY FUCKING GOD");
    // warn!("Something went wrong a little xd");
    // info!("Overflowing line test Overflowing line test Overflowing line test Overflowing line test Overflowing line test Overflowing line test Overflowing line test woo");
    // warn!("Testing newlines right abo\nut now!!! Yeah haha\nwoo newlines!");

    // warn!("Even formatting works!: {}", 1);

    // panic!("Test panic");

    for i in 0..1024 {
        warn!("big printing {}", i);
    }

    loop {}
}
