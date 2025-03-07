use std::ptr::NonNull;

use crate::components::{Cap, Grow, Index, Insert, Len, Pop, Ptr, Push, Remove, Shrink};

use crate::Sector;

pub struct Manual;

impl crate::components::DefaultIter for Manual {}

impl crate::components::DefaultDrain for Manual {}

impl<T> Sector<Manual, T> {
    /// Pushes to the *sector* when there is enoguh capacity
    ///
    /// # Returns
    ///
    /// `true`  - If pushed successfully
    /// `false` - If capacity is full (the __elem__ wont get pushed)
    pub fn push(&mut self, elem: T) -> bool {
        if self.__cap() == self.__len() {
            false
        } else {
            self.__push(elem);
            true
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.__pop()
    }

    /// Inserts to the *sector* when there is enoguh capacity
    ///
    /// # Returns
    ///
    /// `true`  - If inserted successfully
    /// `false` - If capacity is full (the __elem__ wont get pushed)
    pub fn insert(&mut self, index: usize, elem: T) -> bool {
        if self.__cap() == self.__len() {
            false
        } else {
            self.__insert(index, elem);
            true
        }
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

    /// # Returns
    ///
    /// The actual size the Sector has grown
    pub fn grow(&mut self, cap_to_grow: usize) -> usize {
        // TODO: Is this enough zst handling?
        if cap_to_grow == 0 || size_of::<T>() == 0 || self.__cap() >= isize::MAX as usize {
            return 0;
        }

        // calcs the correct size to grow
        let cap_to_grow = match self.__cap().checked_add(cap_to_grow) {
            Some(_) => cap_to_grow,
            None => isize::MAX as usize - cap_to_grow,
        };

        match self.__try_grow_manually(cap_to_grow) {
            Ok(_) => cap_to_grow,
            Err(_) => 0,
        }
    }

    /// # Returns
    ///
    /// The actual size the Sector has shrunk
    pub fn shrink(&mut self, cap_to_shrink: usize) -> usize {
        // TODO: Is this enough zst handling?
        if cap_to_shrink == 0 || size_of::<T>() == 0 || self.__cap() == 0 {
            return 0;
        }

        let shrink_factor = match self.__cap().checked_sub(cap_to_shrink) {
            Some(_) => cap_to_shrink,
            None => self.__cap(),
        };

        let new_cap = self.__cap() - shrink_factor;
        if new_cap < self.__len() {
            for i in new_cap..self.__len() {
                unsafe { self.__ptr().add(i).drop_in_place() };
            }
        }
        match self.__try_shrink_manually(shrink_factor) {
            Ok(_) => shrink_factor,
            Err(_) => 0,
        }
    }
}

impl<T> Ptr<T> for Sector<Manual, T> {
    fn __ptr(&self) -> NonNull<T> {
        unsafe { self.get_ptr() }
    }

    fn __ptr_set(&mut self, new_ptr: NonNull<T>) {
        unsafe { self.set_ptr(new_ptr) };
    }
}

impl<T> Len for Sector<Manual, T> {
    fn __len(&self) -> usize {
        self.get_len()
    }

    fn __len_set(&mut self, new_len: usize) {
        unsafe { self.set_len(new_len) };
    }
}

impl<T> Cap for Sector<Manual, T> {
    fn __cap(&self) -> usize {
        self.get_cap()
    }

    fn __cap_set(&mut self, new_cap: usize) {
        unsafe { self.set_cap(new_cap) };
    }
}

unsafe impl<T> Grow<T> for Sector<Manual, T> {
    unsafe fn __grow(&mut self, _: usize, _: usize) {}
}

unsafe impl<T> Shrink<T> for Sector<Manual, T> {
    unsafe fn __shrink(&mut self, _: usize, _: usize) {}
}

impl<T> Push<T> for Sector<Manual, T> {}
impl<T> Pop<T> for Sector<Manual, T> {}
impl<T> Insert<T> for Sector<Manual, T> {}
impl<T> Index<T> for Sector<Manual, T> {}
impl<T> Remove<T> for Sector<Manual, T> {}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::components::testing::*;

    #[test]
    fn test_push_and_get() {
        let mut sector: Sector<Manual, i32> = Sector::with_capacity(3);

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
        let mut sector: Sector<Manual, ZeroSizedType> = Sector::with_capacity(2);

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
        let mut sector: Sector<Manual, i32> = Sector::with_capacity(3);

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
        let mut sector: Sector<Manual, ZeroSizedType> = Sector::with_capacity(3);

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
        let mut sector: Sector<Manual, i32> = Sector::with_capacity(3);

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
        let mut sector: Sector<Manual, ZeroSizedType> = Sector::with_capacity(3);

        repeat!(sector.push(ZeroSizedType), 2);
        sector.insert(1, ZeroSizedType);
        assert_eq!(sector.get(0), Some(&ZeroSizedType));
        assert_eq!(sector.get(1), Some(&ZeroSizedType));
        assert_eq!(sector.get(2), Some(&ZeroSizedType));
    }

    #[test]
    fn test_remove() {
        let mut sector: Sector<Manual, i32> = Sector::with_capacity(3);

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
        let mut sector: Sector<Manual, ZeroSizedType> = Sector::with_capacity(3);

        repeat!(sector.push(ZeroSizedType), 3);

        assert_eq!(sector.remove(1), ZeroSizedType);
        assert_eq!(sector.get(0), Some(&ZeroSizedType));
        assert_eq!(sector.get(1), Some(&ZeroSizedType));
        assert_eq!(sector.get(2), None);
        assert_eq!(sector.get(3), None);
    }

    #[test]
    fn test_remove_on_emtpy() {
        let mut sector: Sector<Manual, i32> = Sector::with_capacity(3);

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
        let mut sector: Sector<Manual, ZeroSizedType> = Sector::with_capacity(3);

        repeat!(sector.push(ZeroSizedType), 3);

        assert_eq!(sector.remove(1), ZeroSizedType);
        assert_eq!(sector.get(0), Some(&ZeroSizedType));
        assert_eq!(sector.get(1), Some(&ZeroSizedType));
        assert_eq!(sector.get(2), None);
    }

    #[test]
    fn test_get_mut() {
        let mut sector: Sector<Manual, i32> = Sector::with_capacity(3);

        sector.push(10);
        sector.push(20);
        sector.push(30);

        if let Some(value) = sector.get_mut(1) {
            *value = 25;
        }

        assert_eq!(sector.get(1), Some(&25));
    }

    #[test]
    fn test_empty_behavior() {
        let mut sector: Sector<Manual, i32> = Sector::new();

        assert_eq!(sector.pop(), None);
        assert_eq!(sector.get(0), None);
    }

    #[test]
    fn test_empty_behavior_zst() {
        let mut sector: Sector<Manual, ZeroSizedType> = Sector::new();

        assert_eq!(sector.pop(), None);
        assert_eq!(sector.get(0), None);
    }

    #[test]
    fn test_out_of_bounds_access() {
        let mut sector: Sector<Manual, i32> = Sector::with_capacity(1);

        sector.push(10);

        assert_eq!(sector.get(1), None);
        assert_eq!(sector.get_mut(1), None);
    }

    #[test]
    fn test_out_of_bounds_access_zst() {
        let mut sector: Sector<Manual, ZeroSizedType> = Sector::with_capacity(1);

        sector.push(ZeroSizedType);

        assert_eq!(sector.get(1), None);
        assert_eq!(sector.get_mut(1), None);
    }

    #[test]
    fn test_deref() {
        let mut sector: Sector<Manual, i32> = Sector::with_capacity(5);
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
        let mut sector: Sector<Manual, ZeroSizedType> = Sector::with_capacity(5);

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
        let mut sector: Sector<Manual, i32> = Sector::with_capacity(5);
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
        let mut sector: Sector<Manual, ZeroSizedType> = Sector::with_capacity(5);
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
        let mut sector: Sector<Manual, i32> = Sector::with_capacity(6);
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
        let mut sector: Sector<Manual, ZeroSizedType> = Sector::with_capacity(6);
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
        let mut sector: Sector<Manual, i32> = Sector::with_capacity(6);
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
        let mut sector: Sector<Manual, ZeroSizedType> = Sector::with_capacity(6);

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
        let mut sector: Sector<Manual, i32> = Sector::with_capacity(3);

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
        let mut sector: Sector<Manual, i32> = Sector::with_capacity(3);

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
        let mut sector: Sector<Manual, ZeroSizedType> = Sector::with_capacity(3);

        repeat!(sector.push(ZeroSizedType), 3);

        let mut drain_iter = sector.drain();

        assert_eq!(drain_iter.next(), Some(ZeroSizedType));
        assert_eq!(drain_iter.next(), Some(ZeroSizedType));
        assert_eq!(drain_iter.next(), Some(ZeroSizedType));
        assert_eq!(drain_iter.next(), None);
    }

    #[test]
    fn test_drain_next_back() {
        let mut sector: Sector<Manual, i32> = Sector::with_capacity(3);

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
        let mut sector: Sector<Manual, ZeroSizedType> = Sector::with_capacity(3);

        repeat!(sector.push(ZeroSizedType), 3);

        let mut drain_iter = sector.drain();

        assert_eq!(drain_iter.next_back(), Some(ZeroSizedType));
        assert_eq!(drain_iter.next_back(), Some(ZeroSizedType));
        assert_eq!(drain_iter.next_back(), Some(ZeroSizedType));
        assert_eq!(drain_iter.next_back(), None);
    }

    #[test]
    fn test_drain_mixed() {
        let mut sector: Sector<Manual, i32> = Sector::with_capacity(4);

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
        let mut sector: Sector<Manual, i32> = Sector::with_capacity(5);

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
            let mut sector: Sector<Manual, DropCounter> = Sector::with_capacity(5);
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
    fn test_behaviour_grow_1() {
        let mut sector: Sector<Manual, i32> = Sector::with_capacity(100);

        assert_eq!(sector.get_len(), 0);
        assert!(sector.get_cap() == 100);

        for i in 0..100 {
            assert!(sector.push(i));
        }

        assert_eq!(sector.get_len(), 100);
        assert!(sector.get_cap() == 100);
    }

    #[test]
    fn test_behaviour_grow_2() {
        let mut sector: Sector<Manual, i32> = Sector::new();
        repeat!(assert_eq!(sector.pop(), None), 10);
        assert_eq!(sector.grow(10), 10);
        assert_eq!(sector.get_cap(), 10);
    }

    #[test]
    fn test_behaviour_grow_3() {
        let mut sector: Sector<Manual, i32> = Sector::with_capacity(19);
        assert_eq!(sector.get_cap(), 19);

        assert!(sector.push(1));
        assert_eq!(sector.get_cap(), 19);

        assert!(sector.push(2));
        assert_eq!(sector.get_cap(), 19);

        assert!(sector.push(3));
        assert_eq!(sector.get_cap(), 19);

        assert!(sector.push(4));
        assert_eq!(sector.get_cap(), 19);

        assert!(sector.push(5));
        assert_eq!(sector.get_cap(), 19);

        assert!(sector.push(6));
        assert_eq!(sector.get_cap(), 19);

        assert!(sector.push(7));
        assert_eq!(sector.get_cap(), 19);

        assert!(sector.push(8));
        assert_eq!(sector.get_cap(), 19);

        assert!(sector.push(9));
        assert_eq!(sector.get_cap(), 19);

        repeat!(assert!(sector.push(10)), 10);
        assert_eq!(sector.get_cap(), 19);

        // Should now allow pushing -> Cap reached
        repeat!(assert!(!sector.push(10)), 10);

        // Should set the new cap to 19 + 50
        repeat!(assert_eq!(sector.grow(5), 5), 10);
        // Should fill up cap
        repeat!(assert!(sector.push(50)), 50);
        // Should reject new push
        assert!(!sector.push(51));
        // Should set the new cap to 69 + 31
        repeat!(assert_eq!(sector.grow(1), 1), 31);
        // Should fill up cap
        repeat!(assert!(sector.push(31)), 31);
        // Should reject new push
        assert!(!sector.push(51));
    }

    #[test]
    fn test_grow_behavior_zst() {
        let mut sector: Sector<Manual, ZeroSizedType> = Sector::with_capacity(100);

        for _ in 0..100 {
            assert!(sector.push(ZeroSizedType));
        }

        assert_eq!(sector.get_len(), 100);
        assert!(sector.get_cap() == !0);
    }

    #[test]
    fn test_behaviour_shrink_1() {
        let mut sector: Sector<Manual, i32> = Sector::with_capacity(1000);
        assert_eq!(sector.get_cap(), 1000);

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

        assert_eq!(sector.get_cap(), 1000);

        sector.pop();
        sector.pop();
        sector.pop();
        sector.pop();
        sector.pop();

        repeat!(sector.pop(), 994);
        assert_eq!(sector.get_cap(), 1000);

        sector.pop();
        assert_eq!(sector.get_cap(), 1000);
        assert_eq!(sector.get_len(), 0)
    }

    #[test]
    fn test_behaviour_shrink_2() {
        let mut sector: Sector<Manual, i32> = Sector::with_capacity(1000);
        assert_eq!(sector.get_cap(), 1000);

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

        assert_eq!(sector.get_cap(), 1000);

        assert_eq!(sector.shrink(100), 100);

        assert_eq!(sector.get_cap(), 900);

        assert_eq!(sector.shrink(100), 100);

        assert_eq!(sector.get_cap(), 800);

        assert_eq!(sector.shrink(100), 100);

        assert_eq!(sector.get_cap(), 700);

        assert_eq!(sector.shrink(100), 100);

        assert_eq!(sector.get_cap(), 600);

        assert_eq!(sector.shrink(100), 100);

        assert_eq!(sector.get_cap(), 500);

        assert_eq!(sector.shrink(100), 100);

        assert_eq!(sector.get_cap(), 400);

        assert_eq!(sector.shrink(100), 100);

        assert_eq!(sector.get_cap(), 300);

        assert_eq!(sector.shrink(100), 100);

        assert_eq!(sector.get_cap(), 200);

        assert_eq!(sector.shrink(100), 100);

        assert_eq!(sector.get_cap(), 100);

        assert_eq!(sector.shrink(100), 100);

        assert_eq!(sector.get_cap(), 0);

        assert_eq!(sector.shrink(100), 0);
    }

    #[test]
    fn test_behaviour_shrink_3() {
        let mut sector: Sector<Manual, i32> = Sector::new();
        assert_eq!(sector.get_cap(), 0);

        assert_eq!(sector.shrink(100), 0);
        assert_eq!(sector.shrink(1000), 0);
        assert_eq!(sector.shrink(!0), 0);

        assert!(!sector.push(1));
        assert!(!sector.push(1));
        assert!(!sector.push(1));
        assert!(!sector.push(1));
        assert!(!sector.push(1));
        assert!(!sector.push(1));
        assert!(!sector.push(1));
        assert!(!sector.push(1));
        assert!(!sector.push(1));
        assert!(!sector.push(1));
        assert!(!sector.push(1));
        assert!(!sector.push(1));

        assert_eq!(sector.get_cap(), 0);
    }
}
