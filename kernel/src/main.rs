#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_trait_impl)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate log;
#[macro_use] extern crate target_lexicon;

use core::arch::asm;

use bootloader_api::{
    entry_point,
    BootInfo,
    config::{
        BootloaderConfig,
        Mapping
    },
};

use x86_64::{VirtAddr, PhysAddr};
use x86_64::structures::paging::{Page, PageTableFlags};
use x86_64::structures::paging::{Mapper, Size4KiB};

use raw_cpuid::CpuId;

use kernel_common::task_system::{
    executor::SimpleExecutor,
    spawner::Spawner,
};

mod util;
mod panic_handler;
mod framebuffer;
mod logger;
mod gdt;
mod interrupts;
mod memory;
mod heap;
mod wasm;
// mod ring3;

// const WASM_TEST: &'static [u8] = include_bytes!(env!("WASM_TEST_PATH"));

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

async fn kernel_stage_2_main() {
    info!("Kernel stage 2 started!");
    let pixel_format = match framebuffer::fb_mut().info().pixel_format {
        bootloader_api::info::PixelFormat::Rgb => kernel_async::framebuffer::PixelFormat::Rgb,
        bootloader_api::info::PixelFormat::Bgr => kernel_async::framebuffer::PixelFormat::Bgr,
        bootloader_api::info::PixelFormat::U8 => kernel_async::framebuffer::PixelFormat::U8,
        _ => panic!("Unsupported pixel format!"),
    };
    kernel_async::Kernel::builder()
        // .with_framebuffer(framebuffer::fb_mut().buffer_mut(), framebuffer::fb_mut().width(), framebuffer::fb_mut().height(), framebuffer::fb_mut().info().stride, framebuffer::fb_mut().info().bytes_per_pixel, pixel_format).await
        // .with_logger().await
        .build().await
        .run().await;
}

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    framebuffer::init(&mut boot_info.framebuffer);
    framebuffer::fb_mut().set_clear_color([32,32,32]);
    framebuffer::fb_mut().clear();
    kernel_common::logger::init(log::LevelFilter::max(), &logger::LOGGER);
    info!("Hello kernel! Version: {}", VERSION);

    trace!("fb buffer addr: {:p}", framebuffer::fb_mut().buffer_mut());

    let cpuid = CpuId::new();
    debug!("Running on: {:?}", cpuid.get_vendor_info());

    gdt::init();
    interrupts::init_idt();

    debug!("Mem offset boot_info: {:0X?} (hex)", boot_info.physical_memory_offset);
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().expect("Failed to locate physical memory offset!"));
    debug!("Mem offset selected: {:?}", phys_mem_offset);
    memory::init(phys_mem_offset, &boot_info.memory_regions);
    info!("Memory mapped!");

    heap::init();
    info!("Heap initialized!");

    let spawner = Spawner::new();
    spawner.spawn(kernel_stage_2_main());

    SimpleExecutor::run()
}
