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

        if len_to_add == 0 {
            return;
        }

        let (new_cap, new_layout) = if self.__cap() == 0 {
            (len_to_add, Layout::array::<T>(len_to_add).unwrap())
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
    /// This function __may__ gets called regardless of whether memory actually needs
    /// to grow or not. It is up to the implementation to handle that.
    /// __(This function gets called __BEFORE__ the length was added by the insert/push/...
    /// function. This is the __exact opposite__ behaviour of the `shrink()` function)__
    ///
    /// # Arguments
    ///
    /// - `old_len` is the old length of the sector that is currently set
    /// - `new_len` is the new length of the sector
    ///
    /// # Safety
    ///
    /// This trait hevily relies on the '__grow()' implementation, therefore if the implementation of
    /// the mentioned function is wrong it will cause Undefined Behavior.
    ///
    /// <div class="warning">
    /// **Warning:** Incorrect implementation will cause undefined behavior.
    /// </div>
    unsafe fn __grow(&mut self, old_len: usize, new_len: usize);
}
