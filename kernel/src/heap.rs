use x86_64::VirtAddr;
use x86_64::structures::paging::Page;
use good_memory_allocator::SpinLockedAllocator;
use conquer_once::spin::Once;
use crate::memory;

#[global_allocator]
static ALLOCATOR: SpinLockedAllocator = SpinLockedAllocator::empty();
static HEAP_INITIALIZED: Once = Once::uninit();

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 1024 * 4096;

pub fn is_initialized() -> bool {
    HEAP_INITIALIZED.is_initialized()
}

pub fn init() {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        unsafe {
            memory::map_page(page, None).expect("Failed to map page!");
        }
    }

    unsafe {
        ALLOCATOR.init(HEAP_START, HEAP_SIZE);
    }

    HEAP_INITIALIZED.init_once(|| ());
}
