#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(abi_x86_interrupt)]

#[macro_use] extern crate log;

use bootloader_api::{
    entry_point,
    BootInfo,
    config::{
        BootloaderConfig,
        Mapping
    },
};

use x86_64::{VirtAddr, PhysAddr};
use x86_64::structures::paging::Page;

use raw_cpuid::CpuId;

mod util;
mod panic_handler;
mod framebuffer;
mod logger;
mod gdt;
mod interrupts;
mod memory;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    framebuffer::init(&mut boot_info.framebuffer);
    framebuffer::fb_mut().set_clear_color([32,32,32]);
    framebuffer::fb_mut().clear();
    logger::init(log::LevelFilter::max()).unwrap();
    info!("Hello kernel!");

    let cpuid = CpuId::new();
    debug!("Running on: {:?}", cpuid.get_vendor_info());

    gdt::init();
    interrupts::init_idt();

    debug!("Mem offset boot_info: {:0X?} (hex)", boot_info.physical_memory_offset);
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().expect("Failed to locate physical memory offset!"));
    debug!("Mem offset selected: {:?}", phys_mem_offset);

    let mut memory_mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_regions) };

    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf));
    unsafe {
        memory::map_page(page, &mut memory_mapper, &mut frame_allocator).expect("Failed to map page!");
    }
    trace!("Page mapped!");

    // x86_64::instructions::interrupts::int3();
    let ptr = 0xdeadbeaf as *mut u32;
    unsafe { *ptr = 42; }

    info!("It didn't crash!");

    loop {}
}
