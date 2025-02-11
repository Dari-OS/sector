use std::alloc;
use std::{alloc::Layout, mem, ptr::NonNull};

use super::{Cap, Ptr};

/// **Trait `Shrink<T>`**
///
/// Manages reduction of allocated memory by deallocating unused space.
///
/// # Safety
///
/// This trait hevily relies on the '__shrink()' implementation, therefore if the implementation of
/// the mentioned function is wrong it will cause Undefined Behavior.
///
/// - `__shrink_manually` - Reduces the capacity by a specified amount.
/// - `__shrink` - Placeholder for automatic shrink handling.
pub unsafe trait Shrink<T>: Cap + Ptr<T> {
    /// Manually reduces the allocated memory by a specified number of elements.
    ///
    /// # Arguments
    ///
    /// * `len_to_sub` - The number of elements to reduce from the current capacity.
    ///
    /// # Panics
    ///
    /// - Panics if `len_to_sub` exceeds the current capacity.
    fn __shrink_manually(&mut self, len_to_sub: usize) {
        assert!(mem::size_of::<T>() != 0, "Capacity overflow");
        assert!(len_to_sub <= self.__cap(), "Capacity underflow");

        let new_cap = self.__cap() - len_to_sub;
        let new_layout = Layout::array::<T>(new_cap).unwrap();

        let new_ptr = if new_layout.size() > 0 {
            let old_layout = Layout::array::<T>(self.__cap()).unwrap();
            let old_ptr = self.__ptr().as_ptr() as *mut u8;

            let new_u8_ptr = unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) };

            match NonNull::new(new_u8_ptr as *mut T) {
                Some(ptr) => ptr,
                None => alloc::handle_alloc_error(new_layout),
            }
        } else {
            if self.__cap() > 0 {
                let old_layout = Layout::array::<T>(self.__cap()).unwrap();
                unsafe { alloc::dealloc(self.__ptr().as_ptr() as *mut u8, old_layout);}
            }
            NonNull::dangling()
        };

        self.__ptr_set(new_ptr);
        self.__cap_set(new_cap);
    }

    /// Automatically shrinks the memory when needed.
    ///
    /// This function __may__ gets called regardless of whether memory actually needs
    /// to shrink or not. It is up to the implementation to check that.
    /// __(This function gets called AFTER the length was subtracted by the remove/pop/...
    /// function. This is the __exact opposite__ behaviour of the `grow()` function)__
    ///
    /// # Arguments
    ///
    /// - `old_len` is the old length of the sector befor the removal of the elements
    /// - `new_len` is the new length of the sector after the removal of the elements (current length)
    ///
    /// # Safety
    ///
    /// This trait hevily relies on the '__shrink()' implementation, therefore if the implementation of
    /// the mentioned function is wrong it will cause Undefined Behavior.
    ///
    /// <div class="warning">
    /// **Warning:** Incorrect implementation will cause undefined behavior.
    /// </div>
    unsafe fn __shrink(&mut self, old_len: usize, new_len: usize);
}
