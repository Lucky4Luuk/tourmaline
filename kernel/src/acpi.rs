use x86_64::{
    VirtAddr,
    PhysAddr,
    structures::paging::{
        page::{Page, PageRangeInclusive},
        frame::PhysFrame,
    }
};
use acpi::AcpiHandler as AcpiHandlerTrait;
use acpi::{AcpiTables, PhysicalMapping};

pub fn load_acpi(rsdp_addr: u64) -> AcpiTables<AcpiHandler> {
    unsafe {
        AcpiTables::from_rsdp(AcpiHandler::new(), rsdp_addr as usize).expect("Failed to load acpi table!")
    }
}

#[derive(Clone)]
pub struct AcpiHandler;

impl AcpiHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl AcpiHandlerTrait for AcpiHandler {
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize
    ) -> PhysicalMapping<Self, T> {
        PhysicalMapping::new(
            physical_address,
            core::ptr::NonNull::new_unchecked(physical_address as u64 as *mut T),
            size,
            size,
            Self::new(),
        )
    }

    fn unmap_physical_region<T>(region: &PhysicalMapping<Self, T>) {}
}
