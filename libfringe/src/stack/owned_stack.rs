// This file is part of libfringe, a low-level green threading library.
// Copyright (c) whitequark <whitequark@whitequark.org>
// See the LICENSE file included in this distribution.

use crate::{stack::Stack, STACK_ALIGNMENT};

use alloc::{alloc::alloc, boxed::Box};
use core::{alloc::Layout, slice};

/// OwnedStack holds a non-guarded, heap-allocated stack.
#[derive(Debug)]
pub struct OwnedStack(Box<[u8]>);

impl OwnedStack {
  /// Allocates a new stack with exactly `size` accessible bytes and alignment appropriate
  /// for the current platform using the default Rust allocator.
  pub fn new(size: usize) -> OwnedStack {
    // TODO(cynecx): check size according to MIN_STACK_SIZE
    unsafe {
      let aligned_size = size & !(STACK_ALIGNMENT - 1);
      let ptr = alloc(Layout::from_size_align_unchecked(
        aligned_size,
        STACK_ALIGNMENT,
      ));
      OwnedStack(Box::from_raw(slice::from_raw_parts_mut(ptr, aligned_size)))
    }
  }
}

unsafe impl Stack for OwnedStack {
  #[inline(always)]
  fn base(&self) -> *mut u8 {
    // The slice cannot wrap around the address space, so the conversion from usize
    // to isize will not wrap either.
    let len = self.0.len() as isize;
    unsafe { self.limit().offset(len) }
  }

  #[inline(always)]
  fn limit(&self) -> *mut u8 {
    self.0.as_ptr() as *mut u8
  }
}
