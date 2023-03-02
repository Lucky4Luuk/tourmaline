//! Asynchronous part of the kernel

#![no_std]

#[macro_use] extern crate alloc;
#[macro_use] extern crate log;
#[macro_use] extern crate async_trait;

use kernel_common::task_system::spawner::Spawner;

pub mod framebuffer;
mod logger;
mod abi_impl;
mod services;

const WASM_TEST: &'static [u8] = include_bytes!(env!("WASM_TEST_PATH"));

pub struct KernelBuilder {
    spawner: Spawner,
    processor_id: usize,
    fb_init: bool,
    log_init: bool,
}

impl KernelBuilder {
    pub fn new(spawner: Spawner, processor_id: usize) -> Self {
        Self {
            spawner,
            processor_id,
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
            task_spawner: self.spawner,
            processor_id: self.processor_id,
        }
    }
}

pub struct Kernel {
    task_spawner: Spawner,
    processor_id: usize,
}

impl Kernel {
    pub fn builder(spawner: Spawner, processor_id: usize) -> KernelBuilder {
        KernelBuilder::new(spawner, processor_id)
    }

    fn async_spawn_service(&self) {

    }

    pub async fn run(self) {
        // self.task_spawner.spawn_async(log_printer()).await;
        // self.task_spawner.spawn_async(run_wasm(WASM_TEST)).await;
        // for i in 0..10 {
        //     self.task_spawner.spawn_async(test(self.processor_id, i)).await;
        // }
    }
}

async fn run_wasm(data: &[u8]) {
    let mut wasm_program = kernel_common::wasm::WasmProgram::new(data, &abi_impl::ABI);
    wasm_program.run().await;
}

async fn test(id: usize, i: usize) {
    if i % 2 == 0 { kernel_common::task_system::task::yield_now().await; }
    debug!("[{id}] i: {i}");
}

async fn log_printer() {
    loop {

    }
}
