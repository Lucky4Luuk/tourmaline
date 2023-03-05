//! Asynchronous part of the kernel

#![no_std]

#[macro_use] extern crate alloc;
#[macro_use] extern crate log;
#[macro_use] extern crate async_trait;

use kernel_common::task_system::{
    spawner::Spawner,
    scheduler::scheduler_spawn_task,
};

pub mod framebuffer;
mod logger;
mod abi_impl;
mod services;

// const WASM_TEST: &'static [u8] = include_bytes!("../../wasm_test/target/wasm32-unknown-unknown/release/wasm_test.wasm");
const WASM_TEST: &'static [u8] = include_bytes!("../../wasi_test/target/wasm32-wasi/release/wasi_test.wasm");

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

    async fn spawn_async(&self, task: impl core::future::Future<Output = ()> + Send + 'static) {
        // self.task_spawner.spawn_async(task).await;
        let spawner_idx = scheduler_spawn_task(task);
        info!("Spawning task on spawner {spawner_idx}...");
    }

    pub async fn run(self) {
        if self.processor_id == 0 {
            kernel_common::services::service_manager().add_service(alloc::boxed::Box::new(services::StdoutSyslog));
            self.spawn_async(run_wasm(WASM_TEST)).await;
        }
    }
}

async fn run_wasm(data: &[u8]) {
    let wasm_program = kernel_common::wasm::WasmProgram::new(data, &abi_impl::ABI);
    wasm_program.run().await;
}

async fn test(id: usize, i: usize) {
    if i % 2 == 0 { kernel_common::task_system::task::yield_now().await; }
    debug!("i: {i} - spawned from processor {id}");
}
