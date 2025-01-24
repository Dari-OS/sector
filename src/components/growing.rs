use std::{
    alloc::{self, Layout},
    mem,
    ptr::NonNull,
};

use super::{Cap, Ptr};

/// **Trait `Grow<T>`**
///
/// Manages the growth of allocated memory to accommodate more elements.
///
/// # Safety
///
/// This trait hevily relies on the '__grow()' implementation, therefore if the implementation of
/// the mentioned function is wrong it will causes Undefined Behavior.
///
/// <div class="warning">
/// **Warning:** Implementing [`__grow()`] incorrectly will cause undefined behavior.
/// </div>
pub unsafe trait Grow<T>: Cap + Ptr<T> {
    /// Manually grows the allocated memory by a specified amount.
    ///
    /// # Arguments
    ///
    /// * `len_to_add` - Number of additional elements for capacity increase.
    ///
    /// # Aborts
    ///
    /// - Aborts if allocation fails.
    fn __grow_manually(&mut self, len_to_add: usize) {
        assert!(mem::size_of::<T>() != 0, "Capacity overflow");

        let (new_cap, new_layout) = if self.__cap() == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = self.__cap() + len_to_add;
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        let new_ptr = if self.__cap() == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            unsafe {
                let old_ptr = self.__ptr().as_ptr() as *mut u8;
                let old_layout = Layout::array::<T>(self.__cap()).unwrap();
                alloc::realloc(old_ptr, old_layout, new_layout.size())
            }
        };

        self.__ptr_set(match NonNull::new(new_ptr as *mut T) {
            Some(ptr) => ptr,
            None => alloc::handle_alloc_error(new_layout),
        });

        self.__cap_set(new_cap);
    }

    /// Automatically grows the memory when needed.
    ///
    /// This function should be called regardless of whether memory actually needs
    /// to grow or not, as it handles the growth decision internally.
    ///
    /// # Safety
    ///
    /// This trait hevily relies on the '__grow()' implementation, therefore if the implementation of
    /// the mentioned function is wrong it will cause Undefined Behavior.
    ///
    /// <div class="warning">
    /// **Warning:** Incorrect implementation will cause undefined behavior.
    /// </div>
    unsafe fn __grow(&mut self);
}
