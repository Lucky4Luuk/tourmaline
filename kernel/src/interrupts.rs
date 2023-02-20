use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(crate::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
    trace!("IDT enabled!");
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    error!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame); //Don't panic, because the function does not return `!`
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    use x86_64::registers::control::Cr2;
    let addr = Cr2::read();
    error!("EXCEPTION: PAGE FAULT\n{:#?}\nError code: {:?}\nAccessed address: {:?}", stack_frame, error_code, addr);
    // if error_code.is_empty() {
    //     let page = x86_64::structures::paging::Page::containing_address(addr);
    //     unsafe { crate::memory::map_page(page, None).expect("Failed to map page!"); }
    //     debug!("Page now allocated!");
    //     return;
    // }
    loop {} //Halt loop, as we cannot proceed with execution until our page fault has been resolved
}

extern "x86-interrupt" fn general_protection_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    panic!("EXCEPTION: GENERAL PROTECTION FAULT\n{:#?}\nError code: {:?}", stack_frame, error_code);
}
