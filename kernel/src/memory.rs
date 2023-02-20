use x86_64::{VirtAddr, PhysAddr};
use x86_64::structures::paging::{Page, PageTable, OffsetPageTable};

static mut MEMORY_MAPPER: Option<OffsetPageTable> = None;
static mut FRAME_ALLOCATOR: Option<BootInfoFrameAllocator> = None;

pub fn init(phys_mem_offset: VirtAddr, memory_regions: &'static bootloader_api::info::MemoryRegions) {
    let mut memory_mapper = unsafe { init_mapper(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&memory_regions) };
    unsafe {
        MEMORY_MAPPER = Some(memory_mapper);
        FRAME_ALLOCATOR = Some(frame_allocator);
    }
}

pub fn memory_mapper() -> &'static mut OffsetPageTable<'static> {
    unsafe { MEMORY_MAPPER.as_mut().unwrap() }
}

pub fn frame_allocator() -> &'static mut BootInfoFrameAllocator {
    unsafe { FRAME_ALLOCATOR.as_mut().unwrap() }
}

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr)
    -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}

/// Initialize a new OffsetPageTable.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
pub unsafe fn init_mapper(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

use bootloader_api::info::MemoryRegions;

/// A FrameAllocator that returns usable frames from the bootloader's memory map.
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryRegions,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// Create a FrameAllocator from the passed memory map.
    ///
    /// This function is unsafe because the caller must guarantee that the passed
    /// memory map is valid. The main requirement is that all frames that are marked
    /// as `USABLE` in it are really unused.
    pub unsafe fn init(memory_map: &'static MemoryRegions) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }
}

use bootloader_api::info::MemoryRegionKind;
use x86_64::structures::paging::{FrameAllocator, Size4KiB, PhysFrame};

impl BootInfoFrameAllocator {
    /// Returns an iterator over the usable frames specified in the memory map.
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // get usable regions from memory map
        let regions = self.memory_map.iter();
        let usable_regions = regions
            .filter(|r| r.kind == MemoryRegionKind::Usable);
        // map each region to its address range
        let addr_ranges = usable_regions
            .map(|r| r.start..r.end);
        // transform to an iterator of frame start addresses
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        // create `PhysFrame` types from the start addresses
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

use x86_64::structures::paging::mapper::MapToError;
use x86_64::structures::paging::Mapper;
use x86_64::structures::paging::PageTableFlags;

/// Maps a page to a physical frame. Currently marked as unsafe, because I'm unsure of its safety.
/// It shouldn't remap in-use frames, but if it happens, please let me know in a Github issue.
pub unsafe fn map_page(page: Page, extra_flags: Option<PageTableFlags>) -> Result<(), MapToError<Size4KiB>> {
    let frame_allocator = frame_allocator();

    let frame = match frame_allocator.allocate_frame() {
        Some(frame) => frame,
        None => return Err(MapToError::FrameAllocationFailed),
    };

    map_page_to_frame(page, frame, extra_flags)
}

pub unsafe fn map_page_to_frame(page: Page, frame: PhysFrame, extra_flags: Option<PageTableFlags>) -> Result<(), MapToError<Size4KiB>> {
    let mut flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    if let Some(extra_flags) = extra_flags {
        flags |= extra_flags;
    }

    memory_mapper().map_to(page, frame, flags, frame_allocator())?.flush();
    Ok(())
}
