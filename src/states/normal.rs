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
    // Only grows the vec if needed
    unsafe fn __grow(&mut self) {
        if self.get_len() == self.get_cap() {
            self.__grow_manually(self.get_len() + 1);
        }
    }
}

unsafe impl<T> Shrink<T> for Sector<Normal, T> {
    // No shrinking behaviour for the Normal vec
    unsafe fn __shrink(&mut self) {}
}

impl<T> Push<T> for Sector<Normal, T> {}
impl<T> Pop<T> for Sector<Normal, T> {}
impl<T> Insert<T> for Sector<Normal, T> {}
impl<T> Index<T> for Sector<Normal, T> {}
impl<T> Remove<T> for Sector<Normal, T> {}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_empty_behavior() {
        let mut sector: Sector<Normal, i32> = Sector::new();

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
}
