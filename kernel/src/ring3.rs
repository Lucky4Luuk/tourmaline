use core::arch::asm;

// register for address of syscall handler
const MSR_STAR: usize = 0xC0000081;

pub unsafe fn init_syscalls() {
    // write segments to use on syscall/sysret to AMD'S MSR_STAR register
    // asm!("\
    // xor rax, rax
    // mov rdx, 0x230008 // use seg selectors 8, 16 for syscall and 43, 51 for sysret
    // wrmsr" :: "{rcx}"(MSR_STAR) : "rax", "rdx" : "intel", "volatile");
    asm!("\
        xor rax, rax
        mov rdx, 0x230008
        wrmsr",
        in("rcx") MSR_STAR,
    );
}

pub fn user_program() {
    use x86_64::instructions::nop;

    panic!("waaah");

    unsafe { asm!("cli"); }

    nop();
    nop();
    nop();
}

pub unsafe fn jump_usermode(code: x86_64::VirtAddr, stack_end: x86_64::VirtAddr) {
    use x86_64::instructions::segmentation::{CS, DS};
    use x86_64::instructions::segmentation::Segment;
    use crate::gdt::GDT;

    trace!("Jumping to user mode!");

    // CS::set_reg(GDT.1.user_code_selector);
    DS::set_reg(GDT.1.user_data_selector);

    x86_64::instructions::tlb::flush_all();

    let cs_idx = GDT.1.user_code_selector.0;
    let ds_idx = GDT.1.user_data_selector.0;

    asm!("\
        push rax
        push rsi
        push 0x200
        push rdx
        push rdi
        iretq",
        in("rdi") code.as_u64(),
        in("rsi") stack_end.as_u64(),
        in("dx") cs_idx,
        in("ax") ds_idx,
    );
}
