#![no_std]
#![no_main]

#![feature(panic_info_message)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_trait_impl)]
#![feature(asm_const)]
#![feature(const_slice_from_raw_parts_mut)]
#![feature(const_mut_refs)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate log;
// #[macro_use] extern crate target_lexicon;
/// To avoid import collisions with our acpi module, we import it specifically as acpi_crate
extern crate acpi as acpi_crate;

use raw_cpuid::CpuId;

use kernel_common::task_system::{
    executor::SimpleExecutor,
    spawner::Spawner,
    scheduler::scheduler_add_spawner,
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

    // Loading the ACPI tables works fine, but initializing the APIC currently fails.
    // Probably a mistake in my memory allocator, but I don't yet need the APIC anyway.
    static RSDP_REQUEST: LimineRsdpRequest = LimineRsdpRequest::new(0);
    let rsdp_addr = RSDP_REQUEST.get_response().get().unwrap().address.as_ptr().unwrap() as u64;
    let acpi_tables = acpi::load_acpi(rsdp_addr);
    let platform_info = acpi_tables.platform_info().expect("Failed to read platform info!");
    debug!("Processors found: {}", platform_info.processor_info.as_ref().map(|pi| pi.application_processors.len() + 1).unwrap_or(1));
    /*
    if let acpi_crate::InterruptModel::Apic(apic) = &platform_info.interrupt_model {
        apic::init(apic.local_apic_address);
    } else {
        panic!("Unsupported interrupt model! Only APIC is currently supported.");
    }
    */

    let bsp_processor_id = platform_info.processor_info.as_ref().map(|pi| pi.boot_processor.processor_uid).unwrap();
    static SMP_REQUEST: LimineSmpRequest = LimineSmpRequest::new(0).flags(1);
    if let Some(smp_response) = SMP_REQUEST.get_response().get_mut() {
        info!("SMP cpus: {}", smp_response.cpus().len());
        let mut main_cpu_info = None;
        for (i, cpu) in smp_response.cpus().iter_mut().enumerate() {
            cpu.extra_argument = i as u64;
            if cpu.processor_id != bsp_processor_id {
                cpu.goto_address = smp_main;
            } else {
                main_cpu_info = Some(cpu);
            }
        }
        smp_main(main_cpu_info.unwrap().as_ptr())
    } else {
        panic!("SMP could not be enabled!");
    }
}

#[no_mangle]
extern "C" fn smp_main(info: *const LimineSmpInfo) -> ! {
    let info: &'static LimineSmpInfo = unsafe { info.as_ref().unwrap() };
    let processor_id = info.extra_argument as usize;
    info!("Hello from cpu {}!", processor_id);

    // Create the async executor for this core
    let executor = SimpleExecutor::new();
    let spawner = executor.spawner();
    // Add the spawner for this core to the global scheduler
    scheduler_add_spawner(spawner.clone());
    // Spawn the kernel_stage_2_main task on the current core
    spawner.spawn(kernel_stage_2_main(spawner.clone(), processor_id));
    // Run the executor
    executor.run()
}

async fn kernel_stage_2_main(spawner: Spawner, processor_id: usize) {
    kernel_async::Kernel::builder(spawner, processor_id)
        .build().await
        .run().await;
}
