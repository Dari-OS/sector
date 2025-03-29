//! # Dynamic State
//!
//! The `Dynamic` state dynamically adjusts its capacity.
//! Its behavior is governed by growth and shrink factors,
//! ensuring that the container grows when needed and shrinks to save memory when possible.
//!
//! ## Overview
//!
//! The state machine is implemented through a series of traits that define how the container:
//!  - **Push/Pop/Insert/Remove:** Manipulate the element list.
//!  - **Pointer and Length Management (Ptr, Len):** Handle the raw pointer and current length.
//!  - **Capacity Management (Cap):** Get and set the underlying allocation capacity.
//!  - **Dynamic Growth (Grow):** Increase capacity when the current storage is full.
//!  - **Dynamic Shrink (Shrink):** Reduce capacity when the number of elements drops significantly.
//!
//! The growth strategy attempts to double (or more) the capacity when full, while the shrink strategy
//! reduces capacity to roughly 75% of its current value (with a small adjustment) when usage falls
//! below half capacity.

use core::ptr::NonNull;

use crate::components::{Cap, Grow, Index, Insert, Len, Pop, Ptr, Push, Remove, Shrink};

use crate::Sector;

/// The marker type that indicates a dynamic state for a Sector.
///
/// In this state, the sector is implemented as a dynamically resizing vector. The dynamic behavior
/// includes growing the allocation when the number of elements reaches the current capacity, and
/// shrinking the allocation when a significant number of elements are removed. The resizing factors
/// are determined by the following rules:
///
/// - **Growth:** When pushing elements and the current length equals the capacity, the sector will
///   grow its allocation. The new capacity is increased iteratively until it can accommodate the
///   new length. The growth factor is determined by a manual unchecked growth function that
///   typically doubles (or follows a similar pattern) the capacity.
/// - **Shrinkage:** When the sector’s length falls to or below half of its capacity, and the capacity
///   is at least 4, the capacity is reduced. The new capacity is calculated as three-quarters of the
///   old capacity plus a remainder (the modulus of the old capacity by 4). This approach helps to
///   prevent excessive allocation while ensuring a smooth transition when elements are removed.
///
/// This state is intended for use cases where the number of elements is expected to vary significantly.
pub struct Dynamic;

// Provide default iterator and drain behavior.
impl crate::components::DefaultIter for Dynamic {}
impl crate::components::DefaultDrain for Dynamic {}

impl<T> Sector<Dynamic, T> {
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
        if index < self.__len() {
            Some(self.__get(index))
        } else {
            None
        }
    }

    /// Returns a mutable reference to the element at the given index if it exists.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.__len() {
            Some(self.__get_mut(index))
        } else {
            None
        }
    }
}

impl<T> Ptr<T> for Sector<Dynamic, T> {
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

impl<T> Len for Sector<Dynamic, T> {
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

impl<T> Cap for Sector<Dynamic, T> {
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

/// Implements the dynamic growth behavior.
///
/// When adding new elements causes the sector's length to equal its capacity, this method is
/// invoked to allocate a larger block of memory. The growth is done iteratively until the capacity
/// meets or exceeds the new required length.
///
/// # Safety
///
/// The function uses unchecked growth operations. The caller must ensure that the operations
/// do not violate memory safety.
unsafe impl<T> Grow<T> for Sector<Dynamic, T> {
    unsafe fn __grow(&mut self, old_len: usize, new_len: usize) {
        // Check if growth is needed: only when old_len equals current capacity and T is non-zero sized.
        if old_len == self.capacity() && size_of::<T>() != 0 {
            // Grow repeatedly if more than one element was pushed and the new length is not reached yet.
            loop {
                self.__grow_manually_unchecked(if old_len == 0 { 1 } else { old_len });
                if self.__cap() >= new_len {
                    // Stop once the capacity meets or exceeds the new required length.
                    break;
                }
            }
        }
    }
}

/// Implements the dynamic shrink behavior.
///
/// # Algorithm
///
/// The algorithm attempts to shrink the sector when its length is less than or equal to half of
/// its capacity. This is done only if the current capacity is at least 4 and the type is non-zero
/// sized. The new capacity is computed as follows:
///
/// 1. Compute the remainder when dividing the current capacity by 4.
/// 2. Calculate three-quarters of the current capacity (integer division) and add the remainder.
/// 3. The new capacity is the sum of the above two numbers.
///
/// For example, if the capacity was 43:
///  - 43 % 4 = 3 (lost when dividing)
///  - 43 / 4 * 3 = 30 (three quarters of the capacity)
///  - New capacity = 30 + 3 = 33
///
/// # Safety
///
/// The shrink operation is performed using unchecked operations. The caller must ensure that the
/// new capacity is valid and that no memory safety issues arise.
unsafe impl<T> Shrink<T> for Sector<Dynamic, T> {
    unsafe fn __shrink(&mut self, _: usize, new_len: usize) {
        if new_len <= self.__cap() / 2 && self.__cap() >= 4 && size_of::<T>() != 0 {
            let factor_to_add = self.__cap() % 4;
            let new_cap = self.__cap() / 4 * 3 + factor_to_add;
            self.__shrink_manually_unchecked(self.__cap() - new_cap);
        }
    }
}

// The following trait provides additional functionallity based on the grow/shrink
// implementations
// It also serves to mark the available operations on the sector.
impl<T> Push<T> for Sector<Dynamic, T> {}
impl<T> Pop<T> for Sector<Dynamic, T> {}
impl<T> Insert<T> for Sector<Dynamic, T> {}
impl<T> Index<T> for Sector<Dynamic, T> {}
impl<T> Remove<T> for Sector<Dynamic, T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::testing::*;

    #[test]
    fn test_push_and_get() {
        let mut sector: Sector<Dynamic, i32> = Sector::new();

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
        let mut sector: Sector<Dynamic, ZeroSizedType> = Sector::new();

        repeat!(sector.push(ZeroSizedType), 3);

        assert_eq!(sector.get(0), Some(&ZeroSizedType));
        assert_eq!(sector.get(1), Some(&ZeroSizedType));
        assert_eq!(sector.get(2), Some(&ZeroSizedType));
        assert_eq!(sector.get(3), None);
    }

    #[test]
    fn test_pop() {
        let mut sector: Sector<Dynamic, i32> = Sector::new();

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
        let mut sector: Sector<Dynamic, ZeroSizedType> = Sector::new();

        repeat!(sector.push(ZeroSizedType), 3);

        assert_eq!(sector.pop(), Some(ZeroSizedType));
        assert_eq!(sector.pop(), Some(ZeroSizedType));
        assert_eq!(sector.pop(), Some(ZeroSizedType));
        assert_eq!(sector.pop(), None);
    }

    #[test]
    fn test_insert() {
        let mut sector: Sector<Dynamic, i32> = Sector::new();

        sector.push(10);
        sector.push(30);
        sector.insert(1, 20);
        assert_eq!(sector.get(0), Some(&10));
        assert_eq!(sector.get(1), Some(&20));
        assert_eq!(sector.get(2), Some(&30));
    }

    #[test]
    fn test_insert_zst() {
        let mut sector: Sector<Dynamic, ZeroSizedType> = Sector::new();

        repeat!(sector.push(ZeroSizedType), 2);
        sector.insert(1, ZeroSizedType);
        assert_eq!(sector.get(0), Some(&ZeroSizedType));
        assert_eq!(sector.get(1), Some(&ZeroSizedType));
        assert_eq!(sector.get(2), Some(&ZeroSizedType));
    }

    #[test]
    fn test_remove() {
        let mut sector: Sector<Dynamic, i32> = Sector::new();

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
        let mut sector: Sector<Dynamic, ZeroSizedType> = Sector::new();

        repeat!(sector.push(ZeroSizedType), 3);

        assert_eq!(sector.remove(1), ZeroSizedType);
        assert_eq!(sector.get(0), Some(&ZeroSizedType));
        assert_eq!(sector.get(1), Some(&ZeroSizedType));
        assert_eq!(sector.get(2), None);
    }

    #[test]
    fn test_remove_on_emtpy() {
        let mut sector: Sector<Dynamic, i32> = Sector::new();

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
        let mut sector: Sector<Dynamic, ZeroSizedType> = Sector::new();

        repeat!(sector.push(ZeroSizedType), 3);

        assert_eq!(sector.remove(1), ZeroSizedType);
        assert_eq!(sector.get(0), Some(&ZeroSizedType));
        assert_eq!(sector.get(1), Some(&ZeroSizedType));
        assert_eq!(sector.get(2), None);
    }

    #[test]
    fn test_get_mut() {
        let mut sector: Sector<Dynamic, i32> = Sector::new();

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
        let mut sector: Sector<Dynamic, i32> = Sector::new();

        for i in 0..100 {
            sector.push(i);
        }

        assert_eq!(sector.len(), 100);
        assert!(sector.capacity() >= 100);
    }

    #[test]
    fn test_grow_behavior_zst() {
        let mut sector: Sector<Dynamic, ZeroSizedType> = Sector::new();

        for _ in 0..100 {
            sector.push(ZeroSizedType);
        }

        assert_eq!(sector.len(), 100);
        assert!(sector.capacity() >= 100);
    }

    #[test]
    fn test_empty_behavior() {
        let mut sector: Sector<Dynamic, i32> = Sector::new();

        assert_eq!(sector.pop(), None);
        assert_eq!(sector.get(0), None);
    }

    #[test]
    fn test_empty_behavior_zst() {
        let mut sector: Sector<Dynamic, ZeroSizedType> = Sector::new();

        assert_eq!(sector.pop(), None);
        assert_eq!(sector.get(0), None);
    }

    #[test]
    fn test_out_of_bounds_access() {
        let mut sector: Sector<Dynamic, i32> = Sector::new();

        sector.push(10);

        assert_eq!(sector.get(1), None);
        assert_eq!(sector.get_mut(1), None);
    }

    #[test]
    fn test_out_of_bounds_access_zst() {
        let mut sector: Sector<Dynamic, ZeroSizedType> = Sector::new();

        sector.push(ZeroSizedType);

        assert_eq!(sector.get(1), None);
        assert_eq!(sector.get_mut(1), None);
    }

    #[test]
    fn test_deref() {
        let mut sector: Sector<Dynamic, i32> = Sector::new();
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
        let mut sector: Sector<Dynamic, ZeroSizedType> = Sector::new();

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
        let mut sector: Sector<Dynamic, i32> = Sector::new();
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
        let mut sector: Sector<Dynamic, ZeroSizedType> = Sector::new();
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
        let mut sector: Sector<Dynamic, i32> = Sector::new();
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
        let mut sector: Sector<Dynamic, ZeroSizedType> = Sector::new();
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
        let mut sector: Sector<Dynamic, i32> = Sector::new();
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
        let mut sector: Sector<Dynamic, ZeroSizedType> = Sector::new();

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
        let mut sector: Sector<Dynamic, i32> = Sector::new();

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
        let mut sector: Sector<Dynamic, i32> = Sector::new();

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
        let mut sector: Sector<Dynamic, ZeroSizedType> = Sector::new();

        repeat!(sector.push(ZeroSizedType), 3);

        let mut drain_iter = sector.drain();

        assert_eq!(drain_iter.next(), Some(ZeroSizedType));
        assert_eq!(drain_iter.next(), Some(ZeroSizedType));
        assert_eq!(drain_iter.next(), Some(ZeroSizedType));
        assert_eq!(drain_iter.next(), None);
    }

    #[test]
    fn test_drain_next_back() {
        let mut sector: Sector<Dynamic, i32> = Sector::new();

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
        let mut sector: Sector<Dynamic, ZeroSizedType> = Sector::new();

        repeat!(sector.push(ZeroSizedType), 3);

        let mut drain_iter = sector.drain();

        assert_eq!(drain_iter.next_back(), Some(ZeroSizedType));
        assert_eq!(drain_iter.next_back(), Some(ZeroSizedType));
        assert_eq!(drain_iter.next_back(), Some(ZeroSizedType));
        assert_eq!(drain_iter.next_back(), None);
    }

    #[test]
    fn test_drain_mixed() {
        let mut sector: Sector<Dynamic, i32> = Sector::new();

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
        let mut sector: Sector<Dynamic, i32> = Sector::new();

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
            let mut sector: Sector<Dynamic, DropCounter> = Sector::new();
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
        let mut sector: Sector<Dynamic, i32> = Sector::new();
        assert_eq!(sector.capacity(), 0);

        sector.push(1);
        assert_eq!(sector.capacity(), 1);

        sector.push(2);
        assert_eq!(sector.capacity(), 2);

        repeat!(sector.push(3), 2);
        assert_eq!(sector.capacity(), 4);

        repeat!(sector.push(4), 4);
        assert_eq!(sector.capacity(), 8);

        repeat!(sector.push(5), 8);
        assert_eq!(sector.capacity(), 16);

        repeat!(sector.push(6), 16);
        assert_eq!(sector.capacity(), 32);

        repeat!(sector.push(7), 32);
        assert_eq!(sector.capacity(), 64);

        repeat!(sector.push(8), 64);
        assert_eq!(sector.capacity(), 128);

        repeat!(sector.push(9), 128);
        assert_eq!(sector.capacity(), 256);
    }

    //#[test]
    //fn test_behaviour_shrink() {
    //    let mut sector: Sector<Dynamic, i32> = Sector::new();
    //    assert_eq!(sector.get_cap(), 0);
    //
    //    repeat!(sector.push(1), 100);
    //
    //    repeat!(sector.push(2), 100);
    //
    //    repeat!(sector.push(3), 100);
    //
    //    repeat!(sector.push(4), 100);
    //
    //    repeat!(sector.push(5), 100);
    //
    //    repeat!(sector.push(6), 100);
    //
    //    repeat!(sector.push(7), 100);
    //
    //    repeat!(sector.push(8), 100);
    //
    //    repeat!(sector.push(9), 100);
    //
    //    repeat!(sector.push(10), 100);
    //
    //    repeat!(sector.pop(), 1000);
    //    assert_eq!(sector.get_cap(), 1024);
    //}
}
