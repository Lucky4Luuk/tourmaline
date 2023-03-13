use core::ptr::NonNull;
use alloc::alloc::{GlobalAlloc, Layout};
use x86_64::VirtAddr;
use x86_64::structures::paging::Page;
use linked_list_allocator::Heap;
use conquer_once::spin::Once;
use kernel_common::{Mutex, MutexGuard};

pub const HEAP_GROW_SIZE: usize = 64 * 4096 * core::mem::size_of::<usize>();

fn alloc_pages(start: u64, size: u64) {
    let page_range = {
        let heap_start = VirtAddr::new(start);
        let heap_end = heap_start + size - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        unsafe {
            crate::memory::map_page(page, None).expect("Failed to map page!");
        }
    }
}

pub struct LinkedListAlloc {
    inner: Mutex<Heap>,
    init: Once,
}

impl LinkedListAlloc {
    pub const fn empty() -> Self {
        Self {
            inner: Mutex::new(Heap::empty()),
            init: Once::uninit(),
        }
    }

    pub fn was_initialized(&self) -> bool {
        self.init.is_initialized()
    }

    pub unsafe fn init(&self, start: usize, size: usize) {
        if self.was_initialized() { panic!("Cannot initialize heap multiple times!"); }
        self.init.init_once(|| ());
        alloc_pages(start as u64, size as u64);
        self.inner.lock().init(start as u64 as *mut u8, size);
    }

    /// WARNING: Acquires a lock on the allocator!
    unsafe fn grow<'a>(&self, size: usize) {
        let alloc_size = (size.max(HEAP_GROW_SIZE)) as u64;
        alloc_pages(self.inner.lock().top() as u64, alloc_size);
        self.inner.lock().extend(alloc_size as usize);
    }
}

unsafe impl GlobalAlloc for LinkedListAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = match self.inner.lock().allocate_first_fit(layout).map(|nptr| nptr.as_ptr()).ok() {
            Some(ptr) => ptr,
            None => core::ptr::null_mut(),
        };
        if ptr.is_null() {
            self.grow(layout.size());
            self.inner.lock().allocate_first_fit(layout).map(|nptr| nptr.as_ptr()).ok().unwrap_or(core::ptr::null_mut())
        } else {
            ptr
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.inner.lock().deallocate(NonNull::new_unchecked(ptr), layout)
    }
}
