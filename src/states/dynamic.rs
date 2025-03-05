use std::ptr::NonNull;

use crate::components::{Cap, Grow, Index, Insert, Len, Pop, Ptr, Push, Remove, Shrink};

use crate::Sector;

pub struct Dynamic;

impl crate::components::DefaultIter for Dynamic {}

impl crate::components::DefaultDrain for Dynamic {}

impl<T> Sector<Dynamic, T> {
    pub fn push(&mut self, elem: T) {
        self.__push(elem);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.__pop()
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        self.__insert(index, elem);
    }

    pub fn remove(&mut self, index: usize) -> T {
        self.__remove(index)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.__len() {
            Some(self.__get(index))
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.__len() {
            Some(self.__get_mut(index))
        } else {
            None
        }
    }
}

impl<T> Ptr<T> for Sector<Dynamic, T> {
    fn __ptr(&self) -> NonNull<T> {
        unsafe { self.get_ptr() }
    }

    fn __ptr_set(&mut self, new_ptr: NonNull<T>) {
        unsafe { self.set_ptr(new_ptr) };
    }
}

impl<T> Len for Sector<Dynamic, T> {
    fn __len(&self) -> usize {
        self.get_len()
    }

    fn __len_set(&mut self, new_len: usize) {
        unsafe { self.set_len(new_len) };
    }
}

impl<T> Cap for Sector<Dynamic, T> {
    fn __cap(&self) -> usize {
        self.get_cap()
    }

    fn __cap_set(&mut self, new_cap: usize) {
        unsafe { self.set_cap(new_cap) };
    }
}

unsafe impl<T> Grow<T> for Sector<Dynamic, T> {
    unsafe fn __grow(&mut self, old_len: usize, new_len: usize) {
        if old_len == self.get_cap() {
            loop {
                self.__grow_manually(if old_len == 0{  1 } else { old_len });
                if self.__cap() >= new_len {
                    break;
                }
            }
        }
    }
}

/// TODO Document how the simple shrink algo works
unsafe impl<T> Shrink<T> for Sector<Dynamic, T> {
    unsafe fn __shrink(&mut self, _: usize, new_len: usize) {
        if new_len == self.__cap() / 2 && self.__cap() >= 4 {
            let factor_to_add = self.__cap() % 4;
            let new_cap = self.__cap() / 4 * 3 + factor_to_add;
            self.__shrink_manually(self.__cap() - new_cap);
        }
    }
}

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

        assert_eq!(sector.get_len(), 100);
        assert!(sector.get_cap() >= 100);
    }

    #[test]
    fn test_grow_behavior_zst() {
        let mut sector: Sector<Dynamic, ZeroSizedType> = Sector::new();

        for _ in 0..100 {
            sector.push(ZeroSizedType);
        }

        assert_eq!(sector.get_len(), 100);
        assert!(sector.get_cap() >= 100);
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
        let counter = std::cell::Cell::new(0);
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
        assert_eq!(sector.get_cap(), 0);

        sector.push(1);
        assert_eq!(sector.get_cap(), 1);

        sector.push(2);
        assert_eq!(sector.get_cap(), 2);

        repeat!(sector.push(3), 2);
        assert_eq!(sector.get_cap(), 4);

        repeat!(sector.push(4), 4);
        assert_eq!(sector.get_cap(), 8);

        repeat!(sector.push(5), 8);
        assert_eq!(sector.get_cap(), 16);

        repeat!(sector.push(6), 16);
        assert_eq!(sector.get_cap(), 32);

        repeat!(sector.push(7), 32);
        assert_eq!(sector.get_cap(), 64);

        repeat!(sector.push(8), 64);
        assert_eq!(sector.get_cap(), 128);

        repeat!(sector.push(9), 128);
        assert_eq!(sector.get_cap(), 256);
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
