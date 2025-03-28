use core::ptr;

use super::{Cap, Grow, Len, Ptr};

/// **Trait `Push<T>`**
///
/// Adds new elements to the collection, growing if necessary.
///
/// - `__push` - Adds an element at the end of the collection.
pub trait Push<T>: Cap + Len + Ptr<T> + Grow<T> {
    /// Adds an element to the end of the collection.
    ///
    /// # Arguments
    ///
    /// * `elem` - The element to be added.
    ///
    /// # Panics
    ///
    /// - Panics if the `Grow` implementation does not correctly handle growth.
    fn __push(&mut self, elem: T) {
        let len = self.__len();
        self.__len_set(len + 1);
        if len == self.__cap() {
            unsafe { self.__grow(len, len + 1) };
        }

        assert!(len < self.__cap(), "Incorrect Grow implementation");

        unsafe { ptr::write(self.__ptr().as_ptr().add(len), elem) }
    }
}
