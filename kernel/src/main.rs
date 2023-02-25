#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(abi_x86_interrupt)]
#![feature(asm_sym)]
#![feature(alloc_error_handler)]

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

mod util;
mod panic_handler;
mod framebuffer;
mod logger;
mod gdt;
mod interrupts;
mod memory;
mod heap;
mod task_system;
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

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    debug!("async number: {}", number);
}

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    framebuffer::init(&mut boot_info.framebuffer);
    framebuffer::fb_mut().set_clear_color([32,32,32]);
    framebuffer::fb_mut().clear();
    logger::init(log::LevelFilter::max()).unwrap();
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

    let mut executor = task_system::executor::SimpleExecutor::new();
    info!("Executor initialized!");
    executor.spawn(task_system::task::Task::new(example_task()));
    executor.run();

    // info!("Compiling shell...");
    // let shell = wasm::WasmProgram::from_wasm_bytes("shell", SHELL);
    // info!("Shell compiled!");
    // info!("Running shell...");
    // unsafe { shell.run_directly(); }

    loop {}
}
