//! # Tight Sector State
//!
//! The `Tight` state provides a sector implementation that automatically grows or shrinks
//! its capacity by exactly the number of elements required. In contrast to other states that may
//! use multiplicative or fixed reallocation strategies, `Tight` adjusts its allocation
//! precisely to the difference between the new and current lengths.
//!
//! ## Unique Behavior
//!
//! - **Automatic Growth:** When the sector's length reaches its capacity and additional elements
//!   are needed, the sector grows by the exact difference between the new required length and the
//!   current length. This ensures a minimal reallocation strategy.
//!
//! - **Automatic Shrinkage:** When elements are removed and the current length decreases, the sector
//!   shrinks by the precise number of elements removed, releasing any unneeded capacity.
//!
//! All other operations (such as `push`, `pop`, `insert`, and `remove`) behave as in other states.
use core::ptr::NonNull;

use crate::components::{Cap, Grow, Index, Insert, Len, Pop, Ptr, Push, Remove, Shrink};

use crate::Sector;

pub struct Tight;

impl crate::components::DefaultIter for Tight {}

impl crate::components::DefaultDrain for Tight {}

impl<T> Sector<Tight, T> {
    /// Appends an element to the end of the sector.
    ///
    /// # Behavior
    ///
    /// If the current number of elements equals the capacity, the sector will attempt to grow
    /// its storage before inserting the new element.
    pub fn push(&mut self, elem: T) {
        self.__push(elem);
    }

    /// Removes the last element from the sector and returns it.
    ///
    /// Returns `None` if the sector is empty.
    pub fn pop(&mut self) -> Option<T> {
        self.__pop()
    }

    /// Inserts an element at the specified index, shifting all elements after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if the index is greater than the current length.
    pub fn insert(&mut self, index: usize, elem: T) {
        self.__insert(index, elem);
    }

    /// Removes the element at the specified index and returns it, shifting all elements after it to the left.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub fn remove(&mut self, index: usize) -> T {
        self.__remove(index)
    }

    /// Returns a reference to the element at the given index if it exists.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.__get(index)
    }

    /// Returns a mutable reference to the element at the given index if it exists.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.__get_mut(index)
    }
}

impl<T> Ptr<T> for Sector<Tight, T> {
    /// Returns the raw pointer to the first element in the sector.
    ///
    /// # Safety
    ///
    /// The pointer is obtained using an unsafe method which assumes the sector’s storage is valid.
    fn __ptr(&self) -> NonNull<T> {
        unsafe { self.as_ptr() }
    }

    /// Sets the raw pointer of the sector to a new value.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the new pointer is valid for the current sector.
    fn __ptr_set(&mut self, new_ptr: NonNull<T>) {
        unsafe { Sector::set_ptr(self, new_ptr) };
    }
}

impl<T> Len for Sector<Tight, T> {
    /// Returns the current number of elements in the sector.
    fn __len(&self) -> usize {
        Sector::len(self)
    }

    /// Sets the current number of elements in the sector.
    ///
    /// # Safety
    ///
    /// This function is unsafe because the new length must not exceed the actual allocation.
    fn __len_set(&mut self, new_len: usize) {
        unsafe { Sector::set_len(self, new_len) };
    }
}

impl<T> Cap for Sector<Tight, T> {
    /// Returns the current capacity of the sector.
    ///
    /// This value indicates how many elements the sector can hold without needing to grow.
    fn __cap(&self) -> usize {
        self.capacity()
    }

    /// Sets a new capacity for the sector.
    ///
    /// # Safety
    ///
    /// The new capacity must be a valid size for the sector's allocation.
    fn __cap_set(&mut self, new_cap: usize) {
        unsafe { self.set_capacity(new_cap) };
    }
}

/// Implements precise growth behavior for the `Tight` state.
///
/// When the sector's current length equals its capacity and more elements are needed,
/// this implementation grows the sector by exactly `new_len - old_len` elements.
/// This ensures that the capacity is adjusted minimally to accommodate the new elements.
///
/// # Safety
///
/// The function uses unchecked operations. The caller must ensure that these operations
/// do not lead to memory safety issues.
unsafe impl<T> Grow<T> for Sector<Tight, T> {
    unsafe fn __grow(&mut self, old_len: usize, new_len: usize) {
        if old_len == self.capacity() && size_of::<T>() != 0 {
            self.__grow_manually_unchecked(new_len - old_len);
        }
    }
}

/// Implements precise shrink behavior for the `Tight` state.
///
/// When elements are removed and the sector's length decreases,
/// this implementation shrinks the sector by exactly `old_len - new_len` elements,
/// releasing the excess capacity.
///
/// # Safety
///
/// The function uses unchecked operations. The caller must ensure that these operations
/// do not lead to memory safety issues.
unsafe impl<T> Shrink<T> for Sector<Tight, T> {
    unsafe fn __shrink(&mut self, old_len: usize, new_len: usize) {
        if old_len > new_len && size_of::<T>() != 0 {
            self.__shrink_manually_unchecked(old_len - new_len);
        }
    }
}

// The following trait provides additional functionallity based on the grow/shrink
// implementations
// It also serves to mark the available operations on the sector.
impl<T> Push<T> for Sector<Tight, T> {}
impl<T> Pop<T> for Sector<Tight, T> {}
impl<T> Insert<T> for Sector<Tight, T> {}
impl<T> Index<T> for Sector<Tight, T> {}
impl<T> Remove<T> for Sector<Tight, T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::testing::*;

    #[test]
    fn test_push_and_get() {
        let mut sector: Sector<Tight, i32> = Sector::new();

        sector.push(10);
        sector.push(20);
        sector.push(30);

        assert_eq!(sector.get(0), Some(&10));
        assert_eq!(sector.get(1), Some(&20));
        assert_eq!(sector.get(2), Some(&30));
        assert_eq!(sector.get(3), None);
    }

    #[test]
    fn test_push_and_get_zst() {
        let mut sector: Sector<Tight, ZeroSizedType> = Sector::new();

        repeat!(sector.push(ZeroSizedType), 3);

        assert_eq!(sector.get(0), Some(&ZeroSizedType));
        assert_eq!(sector.get(1), Some(&ZeroSizedType));
        assert_eq!(sector.get(2), Some(&ZeroSizedType));
        assert_eq!(sector.get(3), None);
    }

    #[test]
    fn test_pop() {
        let mut sector: Sector<Tight, i32> = Sector::new();

        sector.push(10);
        sector.push(20);
        sector.push(30);

        assert_eq!(sector.pop(), Some(30));
        assert_eq!(sector.pop(), Some(20));
        assert_eq!(sector.pop(), Some(10));
        assert_eq!(sector.pop(), None);
    }

    #[test]
    fn test_pop_zst() {
        let mut sector: Sector<Tight, ZeroSizedType> = Sector::new();

        repeat!(sector.push(ZeroSizedType), 3);

        assert_eq!(sector.pop(), Some(ZeroSizedType));
        assert_eq!(sector.pop(), Some(ZeroSizedType));
        assert_eq!(sector.pop(), Some(ZeroSizedType));
        assert_eq!(sector.pop(), None);
    }

    #[test]
    fn test_insert() {
        let mut sector: Sector<Tight, i32> = Sector::new();

        sector.push(10);
        sector.push(30);
        sector.insert(1, 20);
        assert_eq!(sector.get(0), Some(&10));
        assert_eq!(sector.get(1), Some(&20));
        assert_eq!(sector.get(2), Some(&30));
    }

    #[test]
    fn test_insert_zst() {
        let mut sector: Sector<Tight, ZeroSizedType> = Sector::new();

        repeat!(sector.push(ZeroSizedType), 2);
        sector.insert(1, ZeroSizedType);
        assert_eq!(sector.get(0), Some(&ZeroSizedType));
        assert_eq!(sector.get(1), Some(&ZeroSizedType));
        assert_eq!(sector.get(2), Some(&ZeroSizedType));
    }

    #[test]
    fn test_remove() {
        let mut sector: Sector<Tight, i32> = Sector::new();

        sector.push(10);
        sector.push(20);
        sector.push(30);

        assert_eq!(sector.remove(1), 20);
        assert_eq!(sector.get(0), Some(&10));
        assert_eq!(sector.get(1), Some(&30));
        assert_eq!(sector.get(2), None);
    }

    #[test]
    fn test_remove_zst() {
        let mut sector: Sector<Tight, ZeroSizedType> = Sector::new();

        repeat!(sector.push(ZeroSizedType), 3);

        assert_eq!(sector.remove(1), ZeroSizedType);
        assert_eq!(sector.get(0), Some(&ZeroSizedType));
        assert_eq!(sector.get(1), Some(&ZeroSizedType));
        assert_eq!(sector.get(2), None);
    }

    #[test]
    fn test_remove_on_emtpy() {
        let mut sector: Sector<Tight, i32> = Sector::new();

        sector.push(10);
        sector.push(20);
        sector.push(30);

        assert_eq!(sector.remove(1), 20);
        assert_eq!(sector.get(0), Some(&10));
        assert_eq!(sector.get(1), Some(&30));
        assert_eq!(sector.get(2), None);
    }

    #[test]
    fn test_remove_on_emtpy_zst() {
        let mut sector: Sector<Tight, ZeroSizedType> = Sector::new();

        repeat!(sector.push(ZeroSizedType), 3);

        assert_eq!(sector.remove(1), ZeroSizedType);
        assert_eq!(sector.get(0), Some(&ZeroSizedType));
        assert_eq!(sector.get(1), Some(&ZeroSizedType));
        assert_eq!(sector.get(2), None);
    }

    #[test]
    fn test_get_mut() {
        let mut sector: Sector<Tight, i32> = Sector::new();

        sector.push(10);
        sector.push(20);
        sector.push(30);

        if let Some(value) = sector.get_mut(1) {
            *value = 25;
        }

        assert_eq!(sector.get(1), Some(&25));
    }

    #[test]
    fn test_grow_behavior() {
        let mut sector: Sector<Tight, i32> = Sector::new();

        for i in 0..100 {
            sector.push(i);
        }

        assert_eq!(sector.len(), 100);
        assert!(sector.capacity() >= 100);
    }

    #[test]
    fn test_grow_behavior_zst() {
        let mut sector: Sector<Tight, ZeroSizedType> = Sector::new();

        for _ in 0..100 {
            sector.push(ZeroSizedType);
        }

        assert_eq!(sector.len(), 100);
        assert!(sector.capacity() >= 100);
    }

    #[test]
    fn test_empty_behavior() {
        let mut sector: Sector<Tight, i32> = Sector::new();

        assert_eq!(sector.pop(), None);
        assert_eq!(sector.get(0), None);
    }

    #[test]
    fn test_empty_behavior_zst() {
        let mut sector: Sector<Tight, ZeroSizedType> = Sector::new();

        assert_eq!(sector.pop(), None);
        assert_eq!(sector.get(0), None);
    }

    #[test]
    fn test_out_of_bounds_access() {
        let mut sector: Sector<Tight, i32> = Sector::new();

        sector.push(10);

        assert_eq!(sector.get(1), None);
        assert_eq!(sector.get_mut(1), None);
    }

    #[test]
    fn test_out_of_bounds_access_zst() {
        let mut sector: Sector<Tight, ZeroSizedType> = Sector::new();

        sector.push(ZeroSizedType);

        assert_eq!(sector.get(1), None);
        assert_eq!(sector.get_mut(1), None);
    }

    #[test]
    fn test_deref() {
        let mut sector: Sector<Tight, i32> = Sector::new();
        sector.push(10);
        sector.push(20);
        sector.push(30);
        sector.push(40);
        sector.push(-10);

        let derefed_sec = &*sector;

        assert_eq!(derefed_sec.get(0), Some(&10));
        assert_eq!(derefed_sec.get(1), Some(&20));
        assert_eq!(derefed_sec.get(2), Some(&30));
        assert_eq!(derefed_sec.get(4), Some(&-10));
        assert_eq!(derefed_sec.get(5), None);
    }

    #[test]
    fn test_deref_zst() {
        let mut sector: Sector<Tight, ZeroSizedType> = Sector::new();

        repeat!(sector.push(ZeroSizedType), 5);
        let derefed_sec = &*sector;

        assert_eq!(derefed_sec.get(0), Some(&ZeroSizedType));
        assert_eq!(derefed_sec.get(1), Some(&ZeroSizedType));
        assert_eq!(derefed_sec.get(2), Some(&ZeroSizedType));
        assert_eq!(derefed_sec.get(4), Some(&ZeroSizedType));
        assert_eq!(derefed_sec.get(5), None);
    }

    #[test]
    fn test_deref_mut() {
        let mut sector: Sector<Tight, i32> = Sector::new();
        sector.push(10);
        sector.push(20);
        sector.push(30);
        sector.push(40);
        sector.push(-10);

        let derefed_sec = &mut *sector;

        derefed_sec[0] = 100;
        derefed_sec[1] = 200;
        derefed_sec[4] = -100;

        assert_eq!(derefed_sec.get(0), Some(&100));
        assert_eq!(derefed_sec.get(1), Some(&200));
        assert_eq!(derefed_sec.get(2), Some(&30));
        assert_eq!(derefed_sec.get(4), Some(&-100));
        assert_eq!(derefed_sec.get(5), None);

        assert_eq!(sector.get(0), Some(&100));
        assert_eq!(sector.get(1), Some(&200));
    }

    #[test]
    fn test_deref_mut_zero_sized() {
        let mut sector: Sector<Tight, ZeroSizedType> = Sector::new();
        repeat!(sector.push(ZeroSizedType), 5);

        let derefed_sec = &mut *sector;

        // We can't really update ZSTs...
        assert_eq!(derefed_sec.get(0), Some(&ZeroSizedType));
        assert_eq!(derefed_sec.get(1), Some(&ZeroSizedType));
        assert_eq!(derefed_sec.get(2), Some(&ZeroSizedType));
        assert_eq!(derefed_sec.get(4), Some(&ZeroSizedType));
        assert_eq!(derefed_sec.get(5), None);
    }

    #[test]
    fn test_into_iter_next() {
        let mut sector: Sector<Tight, i32> = Sector::new();
        sector.push(1000);
        sector.push(20528);
        sector.push(3522);
        sector.push(529388);
        sector.push(-81893);
        sector.push(-238146);

        let mut iter_sec = sector.into_iter();

        assert_eq!(iter_sec.next(), Some(1000));
        assert_eq!(iter_sec.next(), Some(20528));
        assert_eq!(iter_sec.next(), Some(3522));
        assert_eq!(iter_sec.next(), Some(529388));
        assert_eq!(iter_sec.next(), Some(-81893));
        assert_eq!(iter_sec.next(), Some(-238146));
        assert_eq!(iter_sec.next(), None);
        assert_eq!(iter_sec.next(), None);
        assert_eq!(iter_sec.next(), None);
    }

    #[test]
    fn test_into_iter_next_zst() {
        let mut sector: Sector<Tight, ZeroSizedType> = Sector::new();
        repeat!(sector.push(ZeroSizedType), 6);

        let mut iter_sec = sector.into_iter();

        assert_eq!(iter_sec.next(), Some(ZeroSizedType));
        assert_eq!(iter_sec.next(), Some(ZeroSizedType));
        assert_eq!(iter_sec.next(), Some(ZeroSizedType));
        assert_eq!(iter_sec.next(), Some(ZeroSizedType));
        assert_eq!(iter_sec.next(), Some(ZeroSizedType));
        assert_eq!(iter_sec.next(), Some(ZeroSizedType));
        assert_eq!(iter_sec.next(), None);
        assert_eq!(iter_sec.next(), None);
        assert_eq!(iter_sec.next(), None);
        assert_eq!(iter_sec.next(), None);
    }

    #[test]
    fn test_into_iter_back() {
        let mut sector: Sector<Tight, i32> = Sector::new();
        sector.push(1000);
        sector.push(20528);
        sector.push(3522);
        sector.push(529388);
        sector.push(-81893);
        sector.push(-238146);

        let mut iter_sec = sector.into_iter();

        assert_eq!(iter_sec.next_back(), Some(-238146));
        assert_eq!(iter_sec.next_back(), Some(-81893));
        assert_eq!(iter_sec.next_back(), Some(529388));
        assert_eq!(iter_sec.next_back(), Some(3522));
        assert_eq!(iter_sec.next_back(), Some(20528));
        assert_eq!(iter_sec.next_back(), Some(1000));
        assert_eq!(iter_sec.next(), None);
        assert_eq!(iter_sec.next(), None);
        assert_eq!(iter_sec.next(), None);
        assert_eq!(iter_sec.next(), None);
    }

    #[test]
    fn test_into_iter_back_zst() {
        let mut sector: Sector<Tight, ZeroSizedType> = Sector::new();

        repeat!(sector.push(ZeroSizedType), 6);

        let mut iter_sec = sector.into_iter();

        assert_eq!(iter_sec.next_back(), Some(ZeroSizedType));
        assert_eq!(iter_sec.next_back(), Some(ZeroSizedType));
        assert_eq!(iter_sec.next_back(), Some(ZeroSizedType));
        assert_eq!(iter_sec.next_back(), Some(ZeroSizedType));
        assert_eq!(iter_sec.next_back(), Some(ZeroSizedType));
        assert_eq!(iter_sec.next_back(), Some(ZeroSizedType));
        assert_eq!(iter_sec.next_back(), None);
        assert_eq!(iter_sec.next_back(), None);
        assert_eq!(iter_sec.next_back(), None);
        assert_eq!(iter_sec.next_back(), None);
    }

    #[test]
    fn test_drain_next() {
        let mut sector: Sector<Tight, i32> = Sector::new();

        sector.push(1);
        sector.push(2);
        sector.push(3);

        let mut drain_iter = sector.drain();

        assert_eq!(drain_iter.next(), Some(1));
        assert_eq!(drain_iter.next(), Some(2));
        assert_eq!(drain_iter.next(), Some(3));
        assert_eq!(drain_iter.next(), None);
    }

    #[test]
    fn test_drain_lifetime() {
        let mut sector: Sector<Tight, i32> = Sector::new();

        sector.push(1);
        sector.push(2);
        sector.push(3);

        let mut drain_iter = sector.drain();

        assert_eq!(drain_iter.next(), Some(1));
        assert_eq!(drain_iter.next(), Some(2));
        assert_eq!(drain_iter.next(), Some(3));
        assert_eq!(drain_iter.next(), None);
    }

    #[test]
    fn test_drain_next_zst() {
        let mut sector: Sector<Tight, ZeroSizedType> = Sector::new();

        repeat!(sector.push(ZeroSizedType), 3);

        let mut drain_iter = sector.drain();

        assert_eq!(drain_iter.next(), Some(ZeroSizedType));
        assert_eq!(drain_iter.next(), Some(ZeroSizedType));
        assert_eq!(drain_iter.next(), Some(ZeroSizedType));
        assert_eq!(drain_iter.next(), None);
    }

    #[test]
    fn test_drain_next_back() {
        let mut sector: Sector<Tight, i32> = Sector::new();

        sector.push(10);
        sector.push(20);
        sector.push(30);

        let mut drain_iter = sector.drain();

        assert_eq!(drain_iter.next_back(), Some(30));
        assert_eq!(drain_iter.next_back(), Some(20));
        assert_eq!(drain_iter.next_back(), Some(10));
        assert_eq!(drain_iter.next_back(), None);
    }

    #[test]
    fn test_drain_next_back_zst() {
        let mut sector: Sector<Tight, ZeroSizedType> = Sector::new();

        repeat!(sector.push(ZeroSizedType), 3);

        let mut drain_iter = sector.drain();

        assert_eq!(drain_iter.next_back(), Some(ZeroSizedType));
        assert_eq!(drain_iter.next_back(), Some(ZeroSizedType));
        assert_eq!(drain_iter.next_back(), Some(ZeroSizedType));
        assert_eq!(drain_iter.next_back(), None);
    }

    #[test]
    fn test_drain_mixed() {
        let mut sector: Sector<Tight, i32> = Sector::new();

        sector.push(100);
        sector.push(200);
        sector.push(300);
        sector.push(400);

        let mut drain_iter = sector.drain();

        assert_eq!(drain_iter.next(), Some(100));
        assert_eq!(drain_iter.next_back(), Some(400));
        assert_eq!(drain_iter.next(), Some(200));
        assert_eq!(drain_iter.next_back(), Some(300));
        assert_eq!(drain_iter.next(), None);
        assert_eq!(drain_iter.next_back(), None);
    }

    #[test]
    fn test_drain_size_hint() {
        let mut sector: Sector<Tight, i32> = Sector::new();

        for i in 0..5 {
            sector.push(i);
        }

        let mut drain_iter = sector.drain();
        let (lower, upper) = drain_iter.size_hint();
        assert_eq!(lower, 5);
        assert_eq!(upper, Some(5));

        drain_iter.next();
        let (lower, upper) = drain_iter.size_hint();
        assert_eq!(lower, 4);
        assert_eq!(upper, Some(4));
    }

    #[test]
    fn test_drain_drop() {
        let counter = core::cell::Cell::new(0);
        {
            let mut sector: Sector<Tight, DropCounter> = Sector::new();
            for _ in 0..5 {
                sector.push(DropCounter { counter: &counter });
            }
            {
                let mut drain_iter = sector.drain();
                assert!(drain_iter.next().is_some());
                assert!(drain_iter.next().is_some());
            }
        }
        assert_eq!(counter.get(), 5);
    }

    #[test]
    fn test_behaviour_grow() {
        let mut sector: Sector<Tight, i32> = Sector::new();
        assert_eq!(sector.capacity(), 0);

        sector.push(1);
        assert_eq!(sector.capacity(), 1);

        sector.push(2);
        assert_eq!(sector.capacity(), 2);

        sector.push(3);
        assert_eq!(sector.capacity(), 3);

        sector.push(4);
        assert_eq!(sector.capacity(), 4);

        sector.push(5);
        assert_eq!(sector.capacity(), 5);

        sector.push(6);
        assert_eq!(sector.capacity(), 6);

        sector.push(7);
        assert_eq!(sector.capacity(), 7);

        sector.push(8);
        assert_eq!(sector.capacity(), 8);

        sector.push(9);
        assert_eq!(sector.capacity(), 9);

        repeat!(sector.push(10), 10);
        assert_eq!(sector.capacity(), 19);
    }

    #[test]
    fn test_behaviour_shrink() {
        let mut sector: Sector<Tight, i32> = Sector::new();
        assert_eq!(sector.capacity(), 0);

        repeat!(sector.push(1), 100);

        repeat!(sector.push(2), 100);

        repeat!(sector.push(3), 100);

        repeat!(sector.push(4), 100);

        repeat!(sector.push(5), 100);

        repeat!(sector.push(6), 100);

        repeat!(sector.push(7), 100);

        repeat!(sector.push(8), 100);

        repeat!(sector.push(9), 100);

        repeat!(sector.push(10), 100);

        assert_eq!(sector.capacity(), 1000);

        sector.pop();
        assert_eq!(sector.capacity(), 999);
        sector.pop();
        assert_eq!(sector.capacity(), 998);
        sector.pop();
        assert_eq!(sector.capacity(), 997);
        sector.pop();
        assert_eq!(sector.capacity(), 996);
        sector.pop();
        assert_eq!(sector.capacity(), 995);

        repeat!(sector.pop(), 994);
        assert_eq!(sector.capacity(), 1);

        sector.pop();
        assert_eq!(sector.capacity(), 0);
    }
}
