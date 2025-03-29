//! # Fixed Sector State
//!
//! The `Fixed` state indicates that the sector has a fixed capacity which does not change once set.
//! This means that operations which would normally trigger a reallocation (growth or shrink) are disabled.
//!
//! In this state, the methods `push` and `insert` will only add an element if there is remaining capacity.
//! They return a boolean value indicating success (`true`) or failure (`false`), depending on whether
//! the operation could be performed without exceeding the fixed capacity.
//!
//! **Note:** There is a known conflict with zero-sized types (ZST). When using a ZST as the element type,
//! a sector with a fixed capacity (e.g., 5) might allow unlimited insertions because ZSTs treat capacity
//! as maximal. This behavior contradicts the intended fixed capacity semantics and is subject to further discussion.
use core::ptr::NonNull;

use crate::components::{Cap, Grow, Index, Insert, Len, Pop, Ptr, Push, Remove, Shrink};

use crate::Sector;

/// The `Fixed` state indicates that the sector has a fixed capacity.
///
/// In this state, operations that would normally trigger a growth or shrink are disabled.
/// Instead, insertions (via `push` or `insert`) only succeed if there is enough capacity already.
///
/// > **Note:** There is a known conflict with zero-sized types (ZST). If a user creates a sector
/// > with a fixed capacity (e.g., 5) and uses a ZST as the element type, it is possible to insert or push
/// > an unlimited number of elements because ZSTs set the capacity to its maximum value. This contradicts
/// > the intended behavior of a fixed-capacity sector. Further discussion or resolution for this issue
/// > is needed.
pub struct Fixed;

impl crate::components::DefaultIter for Fixed {}

impl crate::components::DefaultDrain for Fixed {}

impl<T> Sector<Fixed, T> {
    /// Attempts to push an element to the sector.
    ///
    /// # Behavior
    ///
    /// - If the sector has remaining capacity (i.e. current length is less than capacity),
    ///   the element is pushed and the function returns `true`.
    /// - If the sector is full (current length equals capacity), no element is pushed and the function returns `false`.
    ///
    /// # Returns
    ///
    /// - `true` if the element was successfully pushed.
    /// - `false` if there was insufficient capacity.
    pub fn push(&mut self, elem: T) -> bool {
        if self.__cap() == self.__len() {
            false
        } else {
            self.__push(elem);
            true
        }
    }

    /// Removes the last element from the sector and returns it.
    ///
    /// Returns `None` if the sector is empty.
    pub fn pop(&mut self) -> Option<T> {
        self.__pop()
    }

    /// Attempts to insert an element into the sector at the specified index.
    ///
    /// # Behavior
    ///
    /// - If there is enough capacity (i.e. current length is less than capacity), the element
    ///   is inserted at the provided index and the function returns `true`.
    /// - If the sector is full (current length equals capacity), the insertion is not performed
    ///   and the function returns `false`.
    ///
    /// # Returns
    ///
    /// - `true` if the element was successfully inserted.
    /// - `false` if there was insufficient capacity.
    pub fn insert(&mut self, index: usize, elem: T) -> bool {
        if self.__cap() == self.__len() {
            false
        } else {
            self.__insert(index, elem);
            true
        }
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

impl<T> Ptr<T> for Sector<Fixed, T> {
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

impl<T> Len for Sector<Fixed, T> {
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

impl<T> Cap for Sector<Fixed, T> {
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

/// For the `Fixed` state, the sector is not allowed to grow.
/// This implementation intentionally does nothing.
unsafe impl<T> Grow<T> for Sector<Fixed, T> {
    /// Does nothing as the fixed state does not support growth.
    unsafe fn __grow(&mut self, _: usize, _: usize) {}
}

/// For the `Fixed` state, the sector is not allowed to shrink.
/// This implementation intentionally does nothing.
unsafe impl<T> Shrink<T> for Sector<Fixed, T> {
    /// Does nothing as the fixed state does not support shrinking.
    unsafe fn __shrink(&mut self, _: usize, _: usize) {}
}

// The following trait provides additional functionallity based on the grow/shrink
// implementations
// It also serves to mark the available operations on the sector.
impl<T> Push<T> for Sector<Fixed, T> {}
impl<T> Pop<T> for Sector<Fixed, T> {}
impl<T> Insert<T> for Sector<Fixed, T> {}
impl<T> Index<T> for Sector<Fixed, T> {}
impl<T> Remove<T> for Sector<Fixed, T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::testing::*;

    #[test]
    fn test_push_and_get() {
        let mut sector: Sector<Fixed, i32> = Sector::with_capacity(3);

        sector.push(10);
        sector.push(20);
        sector.push(30);
        assert!(!sector.push(40)); // Should return false because there is no capacity left

        assert_eq!(sector.get(0), Some(&10));
        assert_eq!(sector.get(1), Some(&20));
        assert_eq!(sector.get(2), Some(&30));
        assert_eq!(sector.get(3), None);
        assert_eq!(sector.get(4), None);
    }

    #[test]
    fn test_push_and_get_zst() {
        let mut sector: Sector<Fixed, ZeroSizedType> = Sector::with_capacity(2);

        repeat!(sector.push(ZeroSizedType), 2);

        // Does not work because the cap for ZSTs is a pretty large number
        //assert_eq!(sector.get(0), Some(&ZeroSizedType));
        assert_eq!(sector.get(0), Some(&ZeroSizedType));
        assert_eq!(sector.get(1), Some(&ZeroSizedType));
        assert_eq!(sector.get(2), None);
        assert_eq!(sector.get(3), None);
    }

    #[test]
    fn test_pop() {
        let mut sector: Sector<Fixed, i32> = Sector::with_capacity(3);

        sector.push(10);
        sector.push(20);
        sector.push(30);

        assert_eq!(sector.pop(), Some(30));
        assert_eq!(sector.pop(), Some(20));
        assert_eq!(sector.pop(), Some(10));
        assert_eq!(sector.pop(), None);
        assert_eq!(sector.pop(), None);
        assert_eq!(sector.pop(), None);
    }

    #[test]
    fn test_pop_zst() {
        let mut sector: Sector<Fixed, ZeroSizedType> = Sector::with_capacity(3);

        repeat!(sector.push(ZeroSizedType), 3);

        assert_eq!(sector.pop(), Some(ZeroSizedType));
        assert_eq!(sector.pop(), Some(ZeroSizedType));
        assert_eq!(sector.pop(), Some(ZeroSizedType));
        assert_eq!(sector.pop(), None);
        assert_eq!(sector.pop(), None);
        assert_eq!(sector.pop(), None);
    }

    #[test]
    fn test_insert() {
        let mut sector: Sector<Fixed, i32> = Sector::with_capacity(3);

        sector.push(10);
        sector.push(30);
        sector.insert(1, 20);
        assert!(!sector.insert(1, 20)); // Should return false because there is no capacity left
        assert_eq!(sector.get(0), Some(&10));
        assert_eq!(sector.get(1), Some(&20));
        assert_eq!(sector.get(2), Some(&30));
    }

    #[test]
    fn test_insert_zst() {
        let mut sector: Sector<Fixed, ZeroSizedType> = Sector::with_capacity(3);

        repeat!(sector.push(ZeroSizedType), 2);
        sector.insert(1, ZeroSizedType);
        assert_eq!(sector.get(0), Some(&ZeroSizedType));
        assert_eq!(sector.get(1), Some(&ZeroSizedType));
        assert_eq!(sector.get(2), Some(&ZeroSizedType));
    }

    #[test]
    fn test_remove() {
        let mut sector: Sector<Fixed, i32> = Sector::with_capacity(3);

        sector.push(10);
        sector.push(20);
        sector.push(30);

        assert_eq!(sector.remove(1), 20);
        assert_eq!(sector.get(0), Some(&10));
        assert_eq!(sector.get(1), Some(&30));
        assert_eq!(sector.get(2), None);
        assert_eq!(sector.get(2), None);
    }

    #[test]
    fn test_remove_zst() {
        let mut sector: Sector<Fixed, ZeroSizedType> = Sector::with_capacity(3);

        repeat!(sector.push(ZeroSizedType), 3);

        assert_eq!(sector.remove(1), ZeroSizedType);
        assert_eq!(sector.get(0), Some(&ZeroSizedType));
        assert_eq!(sector.get(1), Some(&ZeroSizedType));
        assert_eq!(sector.get(2), None);
        assert_eq!(sector.get(3), None);
    }

    #[test]
    fn test_remove_on_emtpy() {
        let mut sector: Sector<Fixed, i32> = Sector::with_capacity(3);

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
        let mut sector: Sector<Fixed, ZeroSizedType> = Sector::with_capacity(3);

        repeat!(sector.push(ZeroSizedType), 3);

        assert_eq!(sector.remove(1), ZeroSizedType);
        assert_eq!(sector.get(0), Some(&ZeroSizedType));
        assert_eq!(sector.get(1), Some(&ZeroSizedType));
        assert_eq!(sector.get(2), None);
    }

    #[test]
    fn test_get_mut() {
        let mut sector: Sector<Fixed, i32> = Sector::with_capacity(3);

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
        let mut sector: Sector<Fixed, i32> = Sector::with_capacity(100);

        for i in 0..100 {
            assert!(sector.push(i));
        }

        assert_eq!(sector.len(), 100);
        assert!(sector.capacity() == 100);
    }

    #[test]
    fn test_grow_behavior_zst() {
        let mut sector: Sector<Fixed, ZeroSizedType> = Sector::with_capacity(100);

        for _ in 0..100 {
            assert!(sector.push(ZeroSizedType));
        }

        assert_eq!(sector.len(), 100);
        assert!(sector.capacity() == !0);
    }

    #[test]
    fn test_empty_behavior() {
        let mut sector: Sector<Fixed, i32> = Sector::new();

        assert_eq!(sector.pop(), None);
        assert_eq!(sector.get(0), None);
    }

    #[test]
    fn test_empty_behavior_zst() {
        let mut sector: Sector<Fixed, ZeroSizedType> = Sector::new();

        assert_eq!(sector.pop(), None);
        assert_eq!(sector.get(0), None);
    }

    #[test]
    fn test_out_of_bounds_access() {
        let mut sector: Sector<Fixed, i32> = Sector::with_capacity(1);

        sector.push(10);

        assert_eq!(sector.get(1), None);
        assert_eq!(sector.get_mut(1), None);
    }

    #[test]
    fn test_out_of_bounds_access_zst() {
        let mut sector: Sector<Fixed, ZeroSizedType> = Sector::with_capacity(1);

        sector.push(ZeroSizedType);

        assert_eq!(sector.get(1), None);
        assert_eq!(sector.get_mut(1), None);
    }

    #[test]
    fn test_deref() {
        let mut sector: Sector<Fixed, i32> = Sector::with_capacity(5);
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
        let mut sector: Sector<Fixed, ZeroSizedType> = Sector::with_capacity(5);

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
        let mut sector: Sector<Fixed, i32> = Sector::with_capacity(5);
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
        let mut sector: Sector<Fixed, ZeroSizedType> = Sector::with_capacity(5);
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
        let mut sector: Sector<Fixed, i32> = Sector::with_capacity(6);
        sector.push(1000);
        sector.push(20528);
        sector.push(3522);
        sector.push(529388);
        sector.push(-81893);
        sector.push(-238146);
        assert!(!sector.push(-35892281));

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
        let mut sector: Sector<Fixed, ZeroSizedType> = Sector::with_capacity(6);
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
        let mut sector: Sector<Fixed, i32> = Sector::with_capacity(6);
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
        let mut sector: Sector<Fixed, ZeroSizedType> = Sector::with_capacity(6);

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
        let mut sector: Sector<Fixed, i32> = Sector::with_capacity(3);

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
        let mut sector: Sector<Fixed, i32> = Sector::with_capacity(3);

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
        let mut sector: Sector<Fixed, ZeroSizedType> = Sector::with_capacity(3);

        repeat!(sector.push(ZeroSizedType), 3);

        let mut drain_iter = sector.drain();

        assert_eq!(drain_iter.next(), Some(ZeroSizedType));
        assert_eq!(drain_iter.next(), Some(ZeroSizedType));
        assert_eq!(drain_iter.next(), Some(ZeroSizedType));
        assert_eq!(drain_iter.next(), None);
    }

    #[test]
    fn test_drain_next_back() {
        let mut sector: Sector<Fixed, i32> = Sector::with_capacity(3);

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
        let mut sector: Sector<Fixed, ZeroSizedType> = Sector::with_capacity(3);

        repeat!(sector.push(ZeroSizedType), 3);

        let mut drain_iter = sector.drain();

        assert_eq!(drain_iter.next_back(), Some(ZeroSizedType));
        assert_eq!(drain_iter.next_back(), Some(ZeroSizedType));
        assert_eq!(drain_iter.next_back(), Some(ZeroSizedType));
        assert_eq!(drain_iter.next_back(), None);
    }

    #[test]
    fn test_drain_mixed() {
        let mut sector: Sector<Fixed, i32> = Sector::with_capacity(4);

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
        let mut sector: Sector<Fixed, i32> = Sector::with_capacity(5);

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
            let mut sector: Sector<Fixed, DropCounter> = Sector::with_capacity(5);
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
        let mut sector: Sector<Fixed, i32> = Sector::with_capacity(19);
        assert_eq!(sector.capacity(), 19);

        assert!(sector.push(1));
        assert_eq!(sector.capacity(), 19);

        assert!(sector.push(2));
        assert_eq!(sector.capacity(), 19);

        assert!(sector.push(3));
        assert_eq!(sector.capacity(), 19);

        assert!(sector.push(4));
        assert_eq!(sector.capacity(), 19);

        assert!(sector.push(5));
        assert_eq!(sector.capacity(), 19);

        assert!(sector.push(6));
        assert_eq!(sector.capacity(), 19);

        assert!(sector.push(7));
        assert_eq!(sector.capacity(), 19);

        assert!(sector.push(8));
        assert_eq!(sector.capacity(), 19);

        assert!(sector.push(9));
        assert_eq!(sector.capacity(), 19);

        repeat!(assert!(sector.push(10)), 10);
        assert_eq!(sector.capacity(), 19);
    }

    #[test]
    fn test_behaviour_shrink() {
        let mut sector: Sector<Fixed, i32> = Sector::with_capacity(1000);
        assert_eq!(sector.capacity(), 1000);

        repeat!(assert!(sector.push(1)), 100);

        repeat!(assert!(sector.push(2)), 100);

        repeat!(assert!(sector.push(3)), 100);

        repeat!(assert!(sector.push(4)), 100);

        repeat!(assert!(sector.push(5)), 100);

        repeat!(assert!(sector.push(6)), 100);

        repeat!(assert!(sector.push(7)), 100);

        repeat!(assert!(sector.push(8)), 100);

        repeat!(assert!(sector.push(9)), 100);

        repeat!(assert!(sector.push(10)), 100);

        assert_eq!(sector.capacity(), 1000);

        sector.pop();
        sector.pop();
        sector.pop();
        sector.pop();
        sector.pop();

        repeat!(sector.pop(), 994);
        assert_eq!(sector.capacity(), 1000);

        sector.pop();
        assert_eq!(sector.capacity(), 1000);
        assert_eq!(sector.len(), 0)
    }
}
