#![no_std]
#![no_main]

#![feature(panic_info_message)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_trait_impl)]
#![feature(asm_const)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate log;
// #[macro_use] extern crate target_lexicon;
/// To avoid import collisions with our acpi module, we import it specifically as acpi_crate
extern crate acpi as acpi_crate;

use x86_64::VirtAddr;

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
mod acpi;
mod apic;

use limine::*;

static BOOTLOADER_INFO: LimineBootInfoRequest = LimineBootInfoRequest::new(0);

pub fn hlt_loop() -> ! {
    loop { x86_64::instructions::hlt(); }
}

/// Kernel Entry Point
///
/// `_start` is defined in the linker script as the entry point for the ELF file.
/// Unless the [`Entry Point`](limine::LimineEntryPointRequest) feature is requested,
/// the bootloader will transfer control to this function.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    if let Some(bootinfo) = BOOTLOADER_INFO.get_response().get() {
        kernel_main(bootinfo);
    }
    hlt_loop();
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

async fn kernel_stage_2_main() {
    info!("Kernel stage 2 started!");
    kernel_async::Kernel::builder()
        // .with_framebuffer(framebuffer::fb_mut().buffer_mut(), framebuffer::fb_mut().width(), framebuffer::fb_mut().height(), framebuffer::fb_mut().info().stride, framebuffer::fb_mut().info().bytes_per_pixel, framebuffer::fb_mut().info().pixel_format).await
        // .with_logger().await
        .build().await
        .run().await;
}

fn kernel_main(boot_info: &LimineBootInfoResponse) -> ! {
    framebuffer::init();
    framebuffer::fb_mut().set_clear_color([32,32,32]);
    framebuffer::fb_mut().clear();
    kernel_common::logger::init(log::LevelFilter::max(), &logger::LOGGER);
    info!("Hello kernel! Version: {}", VERSION);
    info!(
        "Booted by {} v{}",
        boot_info.name.to_str().unwrap().to_str().unwrap(),
        boot_info.version.to_str().unwrap().to_str().unwrap(),
    );

    let cpuid = CpuId::new();
    debug!("Running on: {:?}", cpuid.get_vendor_info());

    gdt::init();
    interrupts::init_idt();

    memory::init();
    info!("Memory mapped!");

    heap::init();
    info!("Heap initialized!");

    let rsdp_request = LimineRsdpRequest::new(0);
    let rsdp_addr = rsdp_request.get_response().get().unwrap().address.as_ptr().unwrap() as u64;
    let acpi_tables = acpi::load_acpi(rsdp_addr);
    let platform_info = acpi_tables.platform_info().expect("Failed to read platform info!");
    debug!("Processors found: {}", platform_info.processor_info.as_ref().map(|pi| pi.application_processors.len() + 1).unwrap_or(1));
    if let acpi_crate::InterruptModel::Apic(apic) = &platform_info.interrupt_model {
        apic::init(apic.local_apic_address);
    } else {
        panic!("Unsupported interrupt model! Only APIC is currently supported.");
    }

    let spawner = Spawner::new();
    spawner.spawn(kernel_stage_2_main());

    SimpleExecutor::run()
}
