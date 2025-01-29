use std::ptr::NonNull;

use crate::components::{Cap, Grow, Index, Insert, Len, Pop, Ptr, Push, Remove, Shrink};

use crate::Sector;

pub struct Normal;

impl<T> Sector<Normal, T> {
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

impl<T> Ptr<T> for Sector<Normal, T> {
    fn __ptr(&self) -> NonNull<T> {
        unsafe { self.get_ptr() }
    }

    fn __ptr_set(&mut self, new_ptr: NonNull<T>) {
        unsafe { self.set_ptr(new_ptr) };
    }
}

impl<T> Len for Sector<Normal, T> {
    fn __len(&self) -> usize {
        self.get_len()
    }

    fn __len_set(&mut self, new_len: usize) {
        unsafe { self.set_len(new_len) };
    }
}

impl<T> Cap for Sector<Normal, T> {
    fn __cap(&self) -> usize {
        self.get_cap()
    }

    fn __cap_set(&mut self, new_cap: usize) {
        unsafe { self.set_cap(new_cap) };
    }
}

unsafe impl<T> Grow<T> for Sector<Normal, T> {
    unsafe fn __grow(&mut self, old_len: usize, _: usize) {
        if old_len == self.get_cap() {
            self.__grow_manually(self.get_len() + 1);
        }
    }
}

unsafe impl<T> Shrink<T> for Sector<Normal, T> {
    // No shrinking behaviour for the Normal vec
    unsafe fn __shrink(&mut self, _: usize, _: usize) {
    }
}

impl<T> Push<T> for Sector<Normal, T> {}
impl<T> Pop<T> for Sector<Normal, T> {}
impl<T> Insert<T> for Sector<Normal, T> {}
impl<T> Index<T> for Sector<Normal, T> {}
impl<T> Remove<T> for Sector<Normal, T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct ZeroSizedType;

    impl PartialEq for ZeroSizedType {
        fn eq(&self, _: &Self) -> bool {
            true
        }
    }

    /// Repeats the given expression _n_ times.
    ///
    /// # Example
    ///
    /// This:
    /// ```
    ///
    /// let mut sector: Sector<Normal, ZeroSizedType> = Sector::new();
    /// repeat!(sector.push(ZeroSizedType), 3);
    /// ```
    ///
    /// is equivalent to:
    /// ```
    /// let mut sector: Sector<Normal, ZeroSizedType> = Sector::new();
    ///
    /// sector.push(ZeroSizedType);
    /// sector.push(ZeroSizedType);
    /// sector.push(ZeroSizedType);
    /// ```
    macro_rules! repeat {
        ($ele:expr, $times:expr) => {{
            for _ in 0..$times {
                $ele;
            }
        }};
    }

    #[test]
    fn test_push_and_get() {
        let mut sector: Sector<Normal, i32> = Sector::new();

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
        let mut sector: Sector<Normal, ZeroSizedType> = Sector::new();

        repeat!(sector.push(ZeroSizedType), 3);

        assert_eq!(sector.get(0), Some(&ZeroSizedType));
        assert_eq!(sector.get(1), Some(&ZeroSizedType));
        assert_eq!(sector.get(2), Some(&ZeroSizedType));
        assert_eq!(sector.get(3), None);
    }

    #[test]
    fn test_pop() {
        let mut sector: Sector<Normal, i32> = Sector::new();

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
        let mut sector: Sector<Normal, ZeroSizedType> = Sector::new();

        repeat!(sector.push(ZeroSizedType), 3);

        assert_eq!(sector.pop(), Some(ZeroSizedType));
        assert_eq!(sector.pop(), Some(ZeroSizedType));
        assert_eq!(sector.pop(), Some(ZeroSizedType));
        assert_eq!(sector.pop(), None);
    }

    #[test]
    fn test_insert() {
        let mut sector: Sector<Normal, i32> = Sector::new();

        sector.push(10);
        sector.push(30);
        sector.insert(1, 20);
        assert_eq!(sector.get(0), Some(&10));
        assert_eq!(sector.get(1), Some(&20));
        assert_eq!(sector.get(2), Some(&30));
    }

    #[test]
    fn test_insert_zst() {
        let mut sector: Sector<Normal, ZeroSizedType> = Sector::new();

        repeat!(sector.push(ZeroSizedType), 2);
        sector.insert(1, ZeroSizedType);
        assert_eq!(sector.get(0), Some(&ZeroSizedType));
        assert_eq!(sector.get(1), Some(&ZeroSizedType));
        assert_eq!(sector.get(2), Some(&ZeroSizedType));
    }

    #[test]
    fn test_remove() {
        let mut sector: Sector<Normal, i32> = Sector::new();

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
        let mut sector: Sector<Normal, ZeroSizedType> = Sector::new();

        repeat!(sector.push(ZeroSizedType), 3);

        assert_eq!(sector.remove(1), ZeroSizedType);
        assert_eq!(sector.get(0), Some(&ZeroSizedType));
        assert_eq!(sector.get(1), Some(&ZeroSizedType));
        assert_eq!(sector.get(2), None);
    }

    #[test]
    fn test_remove_on_emtpy() {
        let mut sector: Sector<Normal, i32> = Sector::new();

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
        let mut sector: Sector<Normal, ZeroSizedType> = Sector::new();

        repeat!(sector.push(ZeroSizedType), 3);

        assert_eq!(sector.remove(1), ZeroSizedType);
        assert_eq!(sector.get(0), Some(&ZeroSizedType));
        assert_eq!(sector.get(1), Some(&ZeroSizedType));
        assert_eq!(sector.get(2), None);
    }

    #[test]
    fn test_get_mut() {
        let mut sector: Sector<Normal, i32> = Sector::new();

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
        let mut sector: Sector<Normal, i32> = Sector::new();

        for i in 0..100 {
            sector.push(i);
        }

        assert_eq!(sector.get_len(), 100);
        assert!(sector.get_cap() >= 100);
    }

    #[test]
    fn test_grow_behavior_zst() {
        let mut sector: Sector<Normal, ZeroSizedType> = Sector::new();

        for _ in 0..100 {
            sector.push(ZeroSizedType);
        }

        assert_eq!(sector.get_len(), 100);
        assert!(sector.get_cap() >= 100);
    }

    #[test]
    fn test_empty_behavior() {
        let mut sector: Sector<Normal, i32> = Sector::new();

        assert_eq!(sector.pop(), None);
        assert_eq!(sector.get(0), None);
    }

    #[test]
    fn test_empty_behavior_zst() {
        let mut sector: Sector<Normal, ZeroSizedType> = Sector::new();

        assert_eq!(sector.pop(), None);
        assert_eq!(sector.get(0), None);
    }

    #[test]
    fn test_out_of_bounds_access() {
        let mut sector: Sector<Normal, i32> = Sector::new();

        sector.push(10);

        assert_eq!(sector.get(1), None);
        assert_eq!(sector.get_mut(1), None);
    }

    #[test]
    fn test_out_of_bounds_access_zst() {
        let mut sector: Sector<Normal, ZeroSizedType> = Sector::new();

        sector.push(ZeroSizedType);

        assert_eq!(sector.get(1), None);
        assert_eq!(sector.get_mut(1), None);
    }

    #[test]
    fn test_deref() {

        let mut sector: Sector<Normal, i32> = Sector::new();
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

        let mut sector: Sector<Normal, ZeroSizedType> = Sector::new();

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
       let mut sector: Sector<Normal, i32> = Sector::new();
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
       let mut sector: Sector<Normal, ZeroSizedType> = Sector::new();
       repeat!(sector.push(ZeroSizedType), 5);

       let derefed_sec = &mut *sector;

       // We can't really update ZSTs...
       assert_eq!(derefed_sec.get(0), Some(&ZeroSizedType));
       assert_eq!(derefed_sec.get(1), Some(&ZeroSizedType));
       assert_eq!(derefed_sec.get(2), Some(&ZeroSizedType));
       assert_eq!(derefed_sec.get(4), Some(&ZeroSizedType));
       assert_eq!(derefed_sec.get(5), None);
    }
}
