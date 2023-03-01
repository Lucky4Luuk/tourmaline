use x86_64::{
    VirtAddr,
    PhysAddr,
    structures::paging::{
        page::Page,
        frame::PhysFrame,
    }
};
use x2apic::lapic::{LocalApic, LocalApicBuilder};
use crate::interrupts::LApicInterrupts;

static mut LAPIC: Option<LocalApic> = None;

pub fn init(apic_virtual_address: u64) {
    let page = Page::containing_address(VirtAddr::new(apic_virtual_address));
    let frame = PhysFrame::containing_address(PhysAddr::new(apic_virtual_address));
    unsafe {
        crate::memory::map_page_to_frame(page, frame, None).unwrap();
    }

    let mut lapic = LocalApicBuilder::new()
        .timer_vector(LApicInterrupts::TimerIndex as usize)
        .error_vector(LApicInterrupts::ErrorIndex as usize)
        .spurious_vector(LApicInterrupts::SpuriousIndex as usize)
        .set_xapic_base(apic_virtual_address)
        .build()
        .unwrap_or_else(|err| panic!("{}", err));

    unsafe {
        lapic.enable();
        lapic.enable_timer();

        LAPIC = Some(lapic);
    }
}

pub fn end_of_interrupt() {
    unsafe {
        LAPIC.as_mut().unwrap().end_of_interrupt();
    }
}
