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
mod wasm;
// mod ring3;

const SHELL: &'static [u8] = include_bytes!(env!("SHELL_PATH"));

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

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

    info!("Compiling shell...");
    let shell = wasm::WasmProgram::from_wasm_bytes("shell", SHELL);
    info!("Shell compiled!");
    info!("Running shell...");
    unsafe { shell.run_directly(); }

    // let page = Page::containing_address(VirtAddr::new(0xdeadbeaf));
    // unsafe {
    //     memory::map_page(page, None).expect("Failed to map page!");
    // }
    // trace!("Page mapped!");

    // x86_64::instructions::interrupts::int3();
    // let ptr = 0xdeadbeaf as *mut u32;
    // trace!("Writing out of bounds...");
    // unsafe { *ptr = 42; }
    // trace!("Writing out of bounds again...");
    // unsafe { *ptr = 42; }

    // info!("Setting up user program...");
    // let user_prog_kernel_addr = ring3::user_program as *const() as u64;
    // let kernel_page: Page<Size4KiB> = Page::containing_address(VirtAddr::new(user_prog_kernel_addr));
    // let phys = memory::memory_mapper().translate_page(kernel_page).unwrap();
    // let user_prog_addr = VirtAddr::new(0x400_000);
    // let page = Page::containing_address(user_prog_addr);
    // let offset = user_prog_kernel_addr - kernel_page.start_address().as_u64();
    // debug!("fn addr:   0x{:0X}", user_prog_kernel_addr);
    // debug!("page addr: 0x{:0X}", kernel_page.start_address().as_u64());
    // debug!("offset:    0x{:0X}", offset);
    // let user_prog_addr_fn = user_prog_addr + offset;
    // unsafe {
    //     memory::map_page_to_frame(page, phys, Some(PageTableFlags::USER_ACCESSIBLE)).expect("Failed to map page!");
    // }

    // let mut user_stack: [u8; 0x1000] = [0; 0x1000];
    // let user_stack_ptr = VirtAddr::from_ptr(user_stack.as_ptr());
    // let user_stack_end = user_stack_ptr + 0x1000u64;
    // let kernel_page: Page<Size4KiB> = Page::containing_address(user_stack_ptr);
    // let phys = memory::memory_mapper().translate_page(kernel_page).unwrap();
    // let user_stack_addr = VirtAddr::new(0x800_000);
    // let page = Page::containing_address(user_stack_addr);
    // unsafe {
    //     memory::map_page_to_frame(page, phys, Some(PageTableFlags::USER_ACCESSIBLE)).expect("Failed to map page!");
    // }

    // info!("Ring 3 program:");
    // unsafe {
    //     let ptr = user_prog_addr_fn.as_u64() as *mut [u8; 8];
    //     info!("first 8 user bytes:   {:?}", *ptr);
    //     let ptr = user_prog_kernel_addr as *mut [u8; 8];
    //     info!("first 8 kernel bytes: {:?}", *ptr);
    // }
    //
    // info!("Jumping to ring 3...");
    // unsafe { ring3::jump_usermode(user_prog_addr_fn, user_stack_end); }
    // info!("Ring 3 entered! How did we get here?");

    loop {}
}
