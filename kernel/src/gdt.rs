use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use lazy_static::lazy_static;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

/// The TSS is used to store the Interrupt Stack Table (IST)
// TODO: Stack allocation should be handled by a memory function
lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        // Set stack for ring 0
        // Note the lack of a guard page, so stack overflows could lead to memory corruption
        tss.privilege_stack_table[0] = {
            const STACK_SIZE: usize = 4096;
            static mut RING0_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &RING0_STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };

        // Set up stack for the double fault interrupt
        // Note the lack of a guard page, so stack overflows could lead to memory corruption
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };

        tss
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let kernel_code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let kernel_data_selector = gdt.add_entry(Descriptor::kernel_data_segment());
        let user_code_selector = gdt.add_entry(Descriptor::user_code_segment());
        let user_data_selector = gdt.add_entry(Descriptor::user_data_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors {
            kernel_code_selector,
            kernel_data_selector,
            user_code_selector,
            user_data_selector,
            tss_selector
        })
    };
}

struct Selectors {
    kernel_code_selector: SegmentSelector,
    kernel_data_selector: SegmentSelector,
    user_code_selector: SegmentSelector,
    user_data_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    use x86_64::instructions::segmentation::CS;
    use x86_64::instructions::segmentation::Segment;
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::interrupts::without_interrupts;

    without_interrupts(|| {
        GDT.0.load();
        unsafe {
            CS::set_reg(GDT.1.kernel_code_selector);
            load_tss(GDT.1.tss_selector);
        }
    });
    trace!("GDT enabled!");
}
