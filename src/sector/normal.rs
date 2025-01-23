use std::ptr::NonNull;

use crate::components::{Cap, Grow, Index, Insert, Len, Pop, Ptr, Push, Remove, Shrink};

use super::{Normal, Sector};

impl<'a, T> Sector<'a, T, Normal> {
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

    pub fn get_mut(&'a mut self, index: usize) -> Option<&'a mut T> {
        if index < self.__len() {
            Some(self.__get_mut(index))
        } else {
            None
        }
    }
}

impl<T> Ptr<T> for Sector<'_, T, Normal> {
    fn __ptr(&self) -> NonNull<T> {
        self.buf.ptr
    }

    fn __ptr_set(&mut self, new_ptr: NonNull<T>) {
        self.buf.ptr = new_ptr;
    }
}

impl<T> Len for Sector<'_, T, Normal> {
    fn __len(&self) -> usize {
        self.len
    }

    fn __len_set(&mut self, new_len: usize) {
        self.len = new_len;
    }
}

impl<T> Cap for Sector<'_, T, Normal> {
    fn __cap(&self) -> usize {
        self.buf.cap
    }

    fn __cap_set(&mut self, new_cap: usize) {
        self.buf.cap = new_cap;
    }
}

unsafe impl<T> Grow<T> for Sector<'_, T, Normal> {
    // Only grows the vec if needed
    unsafe fn __grow(&mut self) {
        if self.len == self.buf.cap {
            self.__grow_manually(self.len + 1);
            self.buf.cap += 1;
        }
    }
}

unsafe impl<T> Shrink<T> for Sector<'_, T, Normal> {
    // No shrinking behaviour for the Normal vec
    unsafe fn __shrink(&mut self) {
        return;
    }
}

impl<T> Push<T> for Sector<'_, T, Normal> {}
impl<T> Pop<T> for Sector<'_, T, Normal> {}
impl<T> Insert<T> for Sector<'_, T, Normal> {}
impl<T> Index<T> for Sector<'_, T, Normal> {}
impl<T> Remove<T> for Sector<'_, T, Normal> {}
