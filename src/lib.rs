
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
        let storage: Vec<MaybeUninit<T>> = (0..val).map(|_v| MaybeUninit::uninit()).collect();
        let free_indexes = (0..val).rev().collect();
        Self { storage: storage.into_boxed_slice(), free_indices: free_indexes, capacity: val }
    }

    fn acquire(&mut self, val: T)  -> Option<usize>{
        let idx = self.free_indices.pop()?;
        self.storage[idx] = MaybeUninit::new(val);
        Some(idx)
    }


    #[inline]
    fn release(&mut self, idx: usize) {
        if idx > self.capacity { return }
        unsafe {self.storage[idx].assume_init_drop();}
        self.free_indices.push(idx);
    }

    #[inline]
    fn available(&self) -> usize {
        self.free_indices.len()
    }

    #[inline(always)]
    fn get(&self, idx: usize) -> Option<&T> {
        if idx >= self.capacity { return None; }
        Some(unsafe { self.storage[idx].assume_init_ref()})
    }

    #[inline(always)]
    fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        debug_assert!(idx < self.capacity);
        Some(unsafe { self.storage[idx].assume_init_mut()})
    }

    #[inline]
    fn capacity(&self) -> usize {
        self.capacity
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acquire_and_get() {
        let mut pool:ZPool<u64> = ZPool::new(5);

        assert_eq!(pool.available(), 5);

        let idx = pool.acquire(60).unwrap();

        assert_eq!(*pool.get(idx).unwrap(), 60);
        assert_eq!(pool.available(), 4);
    }
}