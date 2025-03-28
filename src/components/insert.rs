use core::ptr;

use super::{Cap, Grow, Len, Ptr};

/// **Trait `Insert<T>`**
///
/// Inserts an element at a specified index, shifting elements as needed.
///
/// - `__insert` - Inserts an element at the given index.
pub trait Insert<T>: Cap + Len + Ptr<T> + Grow<T> {
    /// Inserts an element at the specified index, shifting elements after it.
    ///
    /// # Arguments
    ///
    /// * `index` - Index to insert at.
    /// * `elem` - Element to insert.
    ///
    /// # Panics
    ///
    /// - Panics if `index` is out of bounds.
    fn __insert(&mut self, index: usize, elem: T) {
        let len = self.__len();
        assert!(index <= len, "Index out of bounds");
        self.__len_set(len + 1);
        if len == self.__cap() {
            unsafe { self.__grow(len, len + 1) };
        }

        assert!(len < self.__cap(), "Incorrect Grow implementation");

        unsafe {
            ptr::copy(
                self.__ptr().as_ptr().add(index),
                self.__ptr().as_ptr().add(index + 1),
                len - index,
            );

            ptr::write(self.__ptr().as_ptr().add(index), elem);
        }
    }
}
