use alloc::alloc::{Layout, GlobalAlloc};
use x86_64::VirtAddr;
use x86_64::structures::paging::Page;
use good_memory_allocator::SpinLockedAllocator;
use kernel_common::Mutex;

use crate::memory;

pub mod linked_list_alloc;
use linked_list_alloc::*;

#[global_allocator]
static ALLOCATOR: LinkedListAlloc = LinkedListAlloc::empty();
// static ALLOCATOR: SpinLockedAllocator = SpinLockedAllocator::empty();

pub const HEAP_START:   usize = 0x_4444_4444_0000;
pub const HEAP_SIZE:    usize = 1024 * 4096;

pub fn is_initialized() -> bool {
    ALLOCATOR.was_initialized()
}

pub fn init() {
    if !is_initialized() {
        unsafe {
            ALLOCATOR.init(HEAP_START, HEAP_SIZE);
        }
    }
}
