use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;
use core::ptr;
use crate::sync::Mutex;

const EWRAM_START: *const u8 = 0x2000000 as *const u8;
#[allow(unused)]
// IWRAM is apparently faster than EWRAM but no clue if I can even use it
const IWRAM_START: *const u8 = 0x03000000 as *const u8;
#[allow(unused)]
// VRAM_START in the event I go through with my vram heap idea
const VRAM_START: *const u8 = 0x06000000 as *const u8;

pub trait GrowHeap {
    fn grow(&self, fit: isize) -> *mut u8;
}

/// global allocator that exclusively uses EWRAM for allocations
/// 
/// GROWTH indicates whether the heap is fixed length or is growable
/// OFFSET is the offset from EWRAM_START (0x2000000) that this allocator grows from
/// LIMIT is the maximum size in bytes of the heap
/// OFFSET + LIMIT cannot exceed 256_000
pub struct EwramAllocator<const GROWTH: bool, const OFFSET: isize, const LIMIT: isize> {
    end: Mutex<*mut u8>
}

impl<const GROWTH: bool, const OFFSET: isize, const LIMIT: isize> EwramAllocator<GROWTH, OFFSET, LIMIT> {
    pub const fn new() -> EwramAllocator<GROWTH, OFFSET, LIMIT> {
        if OFFSET < 0 {
            panic!("heap cannot begin before 0x2000000")
        }
        if LIMIT < 0 {
            panic!("limit cannot be less than 0");
        }
        if OFFSET + LIMIT > 256_000 {
            panic!("offset + limit cannot exceed 256K");
        }
        let end = if GROWTH {
            ptr::null_mut()
        } else {
            unsafe {
                EWRAM_START.offset(LIMIT) as *mut u8
            }
        };
        EwramAllocator {
            end: Mutex::new(end)
        }
    }
}