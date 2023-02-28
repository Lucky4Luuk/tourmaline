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
pub struct AcpiHandler {
    pages: Option<PageRangeInclusive>,
}

impl AcpiHandler {
    pub fn new() -> Self {
        Self {
            pages: None,
        }
    }

    pub fn from_pages(pages: PageRangeInclusive) -> Self {
        Self {
            pages: Some(pages),
        }
    }
}

impl AcpiHandlerTrait for AcpiHandler {
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize
    ) -> PhysicalMapping<Self, T> {
        let page_start = Page::containing_address(VirtAddr::new(physical_address as u64));
        let page_end = Page::containing_address(VirtAddr::new(physical_address as u64 + size as u64));
        let page_range = Page::range_inclusive(page_start, page_end);
        for page in page_range {
            let frame = PhysFrame::containing_address(PhysAddr::new(physical_address as u64));
            crate::memory::map_page_to_frame(page, frame, None).unwrap();
        }
        PhysicalMapping::new(
            physical_address,
            core::ptr::NonNull::new_unchecked(page_start.start_address().as_mut_ptr()),
            size,
            (page_end.start_address().as_u64() - page_start.start_address().as_u64()) as usize,
            Self::from_pages(page_range),
        )
    }

    fn unmap_physical_region<T>(region: &PhysicalMapping<Self, T>) {
        for page in region.handler().pages.unwrap() {
            crate::memory::unmap_page(page);
        }
    }
}
