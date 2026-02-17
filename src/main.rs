

//! custom implementation of a minimal memory allocator
//! here we pre allocate N objects at startup, preventing
//! heap allocation on the hot paths, acquire and release
//! operations are at O(1) generally ...

use std::mem::MaybeUninit;

struct ZPool<T> {
    storage: Box<[MaybeUninit<T>]>,
    free_indices: Vec<usize>,
    capacity: usize
}

impl <T> ZPool<T> {
    fn new(val: usize) -> Self {
        let storage: Vec<MaybeUninit<T>> = (0..val).map(|v| MaybeUninit::uninit()).collect();
        let free_indexes = (0..val).rev().collect();
        Self { storage: storage.into_boxed_slice(), free_indices: free_indexes, capacity: val }
    }
}


fn main() {}