//! Simple memory allocation.
//!
//! TODO: more efficient

use core::alloc::Layout;
use core::num::NonZeroUsize;

use crate::{AllocResult, BaseAllocator, ByteAllocator};

pub struct SimpleByteAllocator{
    cur_ptr:usize,
    total_bytes:usize,
    used_bytes:usize,
}

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {
            cur_ptr: 0,
            total_bytes:0,
            used_bytes:0,
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, _start: usize, _size: usize) {
        self.cur_ptr = _start;
        self.total_bytes = _size;
        self.used_bytes = 0;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        self.cur_ptr = _start;
        self.total_bytes = _size;
        self.used_bytes = 0;
        Ok(())
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, _layout: Layout) -> AllocResult<NonZeroUsize> {
        if (self.total_bytes-self.used_bytes)< _layout.size(){
            Err(crate::AllocError::NotAllocated)
        }else{
            let ptr = self.cur_ptr;
            self.cur_ptr+=_layout.size();
            self.used_bytes+=_layout.size();
            Ok(NonZeroUsize::new(ptr).unwrap())
        }
    }

    fn dealloc(&mut self, _pos: NonZeroUsize, _layout: Layout) {
        // TODO implementation
    }

    fn total_bytes(&self) -> usize {
        self.total_bytes
    }

    fn used_bytes(&self) -> usize {
        self.used_bytes
    }

    fn available_bytes(&self) -> usize {
        self.total_bytes-self.used_bytes
    }
}
