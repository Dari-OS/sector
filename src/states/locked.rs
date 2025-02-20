use std::ptr::NonNull;

use crate::components::{Cap, Grow, Index, Insert, Len, Pop, Ptr, Push, Remove, Shrink};

use crate::Sector;

pub struct Locked;

impl crate::components::DefaultIter for Locked {}

impl crate::components::DefaultDrain for Locked {}

//impl<T> Sector<Locked, T> {
//}

impl<T> Ptr<T> for Sector<Locked, T> {
    fn __ptr(&self) -> NonNull<T> {
        unsafe { self.get_ptr() }
    }

    fn __ptr_set(&mut self, new_ptr: NonNull<T>) {
        unsafe { self.set_ptr(new_ptr) };
    }
}

impl<T> Len for Sector<Locked, T> {
    fn __len(&self) -> usize {
        self.get_len()
    }

    fn __len_set(&mut self, new_len: usize) {
        unsafe { self.set_len(new_len) };
    }
}

impl<T> Cap for Sector<Locked, T> {
    fn __cap(&self) -> usize {
        self.get_cap()
    }

    fn __cap_set(&mut self, new_cap: usize) {
        unsafe { self.set_cap(new_cap) };
    }
}

unsafe impl<T> Grow<T> for Sector<Locked, T> {
    unsafe fn __grow(&mut self, _: usize, _: usize) {}
}

unsafe impl<T> Shrink<T> for Sector<Locked, T> {
    unsafe fn __shrink(&mut self, _: usize, _: usize) {}
}

impl<T> Push<T> for Sector<Locked, T> {}
impl<T> Pop<T> for Sector<Locked, T> {}
impl<T> Insert<T> for Sector<Locked, T> {}
impl<T> Index<T> for Sector<Locked, T> {}
impl<T> Remove<T> for Sector<Locked, T> {}

#[cfg(test)]
mod tests {
    //use super::*;
    //use crate::components::testing::*;

    // TODO: Implemented transiotions to test the locked state (It works but still needs test for
    //consitency)
}
