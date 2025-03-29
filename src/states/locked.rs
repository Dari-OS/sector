//! # Locked Sector State
//!
//! The `Locked` state represents a sector whose capacity is fixed at creation and cannot be modified
//! by growth or shrink operations. This state is useful when a fixed memory layout is required,
//! ensuring that once a sector is created, its capacity remains constant.
//!
//! ## Unique Behavior
//!
//! - **Growth Operations:** The implementation of the [`Grow`] trait for the `Locked` state is a no-op.
//!   Any attempt to grow the sector is silently ignored.
//!
//! - **Shrink Operations:** Similarly, the implementation of the [`Shrink`] trait is a no-op,
//!   meaning that the sector will not attempt to reduce its capacity under any circumstances.
//!
//! All other operations (such as element access, insertion, and removal) behave as defined by their
//! respective traits and do not have unique documentation for the `Locked` state.
use core::ptr::NonNull;

use crate::components::{Cap, Grow, Index, Insert, Len, Pop, Ptr, Push, Remove, Shrink};

use crate::Sector;

pub struct Locked;

impl crate::components::DefaultIter for Locked {}

impl crate::components::DefaultDrain for Locked {}

impl<T> Sector<Locked, T> {
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

impl<T> Ptr<T> for Sector<Locked, T> {
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

impl<T> Len for Sector<Locked, T> {
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

impl<T> Cap for Sector<Locked, T> {
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

/// No-op implementation for growth in the `Locked` state.
///
/// In the `Locked` state, the sector's capacity is immutable and any attempt to grow
/// the sector is ignored.
unsafe impl<T> Grow<T> for Sector<Locked, T> {
    unsafe fn __grow(&mut self, _: usize, _: usize) {}
}

/// No-op implementation for shrinking in the `Locked` state.
///
/// Since the sector is locked to a fixed capacity, this implementation does not perform any
/// shrinking, regardless of the current or new length.
unsafe impl<T> Shrink<T> for Sector<Locked, T> {
    unsafe fn __shrink(&mut self, _: usize, _: usize) {}
}

// The following trait provides additional functionallity based on the grow/shrink
// implementations
// It also serves to mark the available operations on the sector.
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
