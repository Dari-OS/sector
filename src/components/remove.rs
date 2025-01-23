/// **Trait `Remove<T>`**
///
/// Removes an element from a specified index, shifting elements to fill the gap.
///
/// - `__remove` - Removes and returns the element at the index.
pub trait Remove<T>: Cap + Len + Ptr<T> + Shrink<T> {
    /// Removes and returns the element at a specified index, shifting subsequent elements.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the element to remove.
    ///
    /// # Returns
    ///
    /// * `T` - The removed element.
    ///
    /// # Panics
    ///
    /// - Panics if `index` is out of bounds.
    fn __remove(&mut self, index: usize) -> T {
        let len = self.__len();
        assert!(index < len, "Index out of bounds");
        let result = unsafe { ptr::read(self.__ptr().as_ptr().add(index)) };
        unsafe {
            ptr::copy(
                self.__ptr().as_ptr().add(index + 1),
                self.__ptr().as_ptr().add(index),
                len - index,
            );

            // Shrink implementation should handle reducing memory when necessary
            self.__shrink();
            self.__len_set(len - 1)
        }
        result
    }
}
