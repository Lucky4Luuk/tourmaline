//! Asynchronous part of the kernel

#![no_std]

#[macro_use] extern crate alloc;
#[macro_use] extern crate log;

use kernel_common::task_system::spawner::Spawner;

pub mod framebuffer;
mod logger;

pub struct KernelBuilder {
    fb_init: bool,
    log_init: bool,
}

impl KernelBuilder {
    pub fn new() -> Self {
        Self {
            fb_init: false,
            log_init: false,
        }
    }

    pub async fn with_framebuffer(mut self, fb: &'static mut [u8], width: usize, height: usize, stride: usize, bytes_per_pixel: usize, pixel_format: framebuffer::PixelFormat) -> Self {
        framebuffer::init(fb, width, height, stride, bytes_per_pixel, pixel_format).await;
        self.fb_init = true;
        self
    }

    pub async fn with_logger(mut self) -> Self {
        kernel_common::logger::init(log::LevelFilter::max(), &logger::LOGGER);
        self.log_init = true;
        self
    }

    pub async fn build(self) -> Kernel {
        Kernel {
            task_spawner: Spawner::new(),
        }
    }
}

pub struct Kernel {
    task_spawner: Spawner,
}

impl Kernel {
    pub fn builder() -> KernelBuilder {
        KernelBuilder::new()
    }

    pub async fn run(self) {
        // self.task_spawner.spawn_async(log_printer()).await;
        for i in 0..10 {
            self.task_spawner.spawn_async(test(i)).await;
        }
    }
}

async fn test(i: usize) {
    if i % 2 == 0 { kernel_common::task_system::task::yield_now().await; }
    debug!("i: {}", i);
}

async fn log_printer() {
    loop {

    }
}
