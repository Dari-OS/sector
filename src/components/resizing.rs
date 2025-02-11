use std::{
    alloc::{self, Layout},
    ptr::NonNull,
};

use super::{Cap, Ptr};

/// **Trait `Resize<T>`**
///
/// Resizes the allocation to a specified capacity directly.
///
/// - `__resize` - Changes capacity to a given number of elements.
#[allow(dead_code)]
pub trait Resize<T>: Cap + Ptr<T> {
    /// Resizes the allocation to the specified capacity.
    ///
    /// # Arguments
    ///
    /// * `new_cap` - The desired new capacity.
    ///
    /// # Panics
    ///
    /// - Panics if the allocation size exceeds `isize::MAX`.
    fn __resize(&mut self, new_cap: usize) {
        let new_layout = Layout::array::<T>(new_cap).unwrap();
        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Capacity overflow"
        );

        let new_ptr = if self.__cap() == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.__cap()).unwrap();
            let old_ptr = self.__ptr().as_ptr() as *mut u8;

            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        let new_ptr = match NonNull::new(new_ptr as *mut T) {
            Some(ptr) => ptr,
            None => alloc::handle_alloc_error(new_layout),
        };

        self.__cap_set(new_cap);
        self.__ptr_set(new_ptr);
    }
}
