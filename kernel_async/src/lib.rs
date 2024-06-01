//! Asynchronous part of the kernel

#![no_std]

#[macro_use] extern crate alloc;
#[macro_use] extern crate log;
#[macro_use] extern crate async_trait;

use alloc::boxed::Box;

use kernel_common::task_system::{
    spawner::Spawner,
    scheduler::{scheduler_spawn_task, SchedulerService},
};
use kernel_common::services::service_manager;
use kernel_common::requests::*;

mod logger;
mod abi_impl;
mod services;

const WASM_TEST: &'static [u8] = include_bytes!("../../wasi_test/target/wasm32-wasi/release/wasi_test.wasm");

pub struct KernelBuilder {
    spawner: Spawner,
    processor_id: usize,
    fb_init: bool,
    log_init: bool,
}

impl KernelBuilder {
    pub fn new(spawner: Spawner, processor_id: usize) -> Self {
        // let stack = fringe::OwnedStack::new(1 << 16);
        // let mut gen = unsafe { fringe::Generator::unsafe_new(stack, move |yielder, ()| {
        //     test_func();
        //     yielder.suspend(());
        // }) };
        // gen.resume(());
        // gen.resume(());
        Self {
            spawner,
            processor_id,
            fb_init: false,
            log_init: false,
        }
    }

    /// Enables the framebuffer driver
    pub(crate) async fn with_framebuffer(mut self) -> Self {
        use framebuffer_driver::*;

        if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response().get() {
            let fb = &framebuffer_response.framebuffers()[0];
            let rgb_or_bgr = if fb.memory_model == 1 { PixelFormat::Rgb } else { PixelFormat::Bgr };
            // fb.bpp is bits per pixel
            let (bytes_per_pixel, pixel_format) = match fb.bpp {
                8 => (1, PixelFormat::U8),
                16 => (2, PixelFormat::U8),
                24 => (3, rgb_or_bgr),
                32 => (4, rgb_or_bgr),
                _ => unimplemented!(),
            };
            let info = FramebufferInfo {
                width: fb.width as usize,
                height: fb.height as usize,
                stride: fb.pitch as usize,
                bytes_per_pixel,
                pixel_format,
            };
            let fb_len = (info.width + info.height * info.stride) * info.bytes_per_pixel;
            let buf = unsafe { core::slice::from_raw_parts_mut(fb.address.as_ptr().unwrap(), fb_len) };
            service_manager().add_service(Box::new(FramebufferDriver::init(buf, info)));
            self.fb_init = true;
            self
        } else {
            panic!("Failed to initialize framebuffer!");
        }
    }

    pub(crate) async fn with_logger(mut self) -> Self {
        kernel_common::logger::init(log::LevelFilter::max(), &logger::LOGGER);
        self.log_init = true;
        self
    }

    pub async fn build(mut self) -> Kernel {
        self = self.with_framebuffer().await;
        // self = self.with_logger().await;

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
            service_manager().add_service(Box::new(SchedulerService));
            service_manager().add_service(Box::new(services::StdoutSyslog));
            service_manager().add_service(Box::new(services::FileDescriptorManager::new()));
            // self.spawn_async(yield_loop()).await;
            self.spawn_async(run_wasm(WASM_TEST)).await;
        }
    }
}

async fn run_wasm(data: &[u8]) {
    let wasm_program = kernel_common::wasm::WasmProgram::new(data, &abi_impl::ABI);
    wasm_program.run().await;
}

async fn yield_loop() {
    loop {
        kernel_common::task_system::task::yield_now().await;
        info!("yielded!");
        kernel_common::task_system::delay::delay(1).await;
    }
}

async fn test(id: usize, i: usize) {
    if i % 2 == 0 { kernel_common::task_system::task::yield_now().await; }
    debug!("i: {i} - spawned from processor {id}");
}
