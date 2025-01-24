use std::ptr;

use super::{Cap, Len, Ptr, Shrink};

/// **Trait `Pop<T>`**
///
/// Removes the last element from the collection, shrinking if necessary.
///
/// - `__pop` - Removes and returns the last element.
pub trait Pop<T>: Cap + Len + Ptr<T> + Shrink<T> {
    /// Removes and returns the last element from the collection.
    ///
    /// # Returns
    ///
    /// * `Option<T>` - Returns the removed element, or `None` if empty.
    fn __pop(&mut self) -> Option<T> {
        // The shrink implementation should handle whether or not to shrink the underlying pointer
        unsafe { self.__shrink() };
        let len = self.__len();
        if len == 0 {
            None
        } else {
            self.__len_set(len - 1);
            Some(unsafe { ptr::read(self.__ptr().as_ptr().add(len - 1)) })
        }
    }
}
