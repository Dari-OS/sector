use super::{Len, Ptr};

/// **Trait `Index<T>`**
///
/// Provides access to elements by index for reading and writing.
///
/// - `__get` - Retrieves a reference to an element by index.
/// - `__get_mut` - Retrieves a mutable reference to an element by index.
pub trait Index<T>: Len + Ptr<T> {
    /// Retrieves a reference to an element at a specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the element to retrieve.
    ///
    /// # Returns
    ///
    /// * `&T` - Reference to the element.
    ///
    /// # Panics
    ///
    /// - Panics if `index` is out of bounds.
    fn __get(&self, index: usize) -> &T {
        assert!(index <= self.__len(), "Index out of bounds");
        unsafe { &*self.__ptr().as_ptr().add(index) }
    }

    /// Retrieves a mutable reference to an element at a specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the element to retrieve.
    ///
    /// # Returns
    ///
    /// * `&mut T` - Mutable reference to the element.
    ///
    /// # Panics
    ///
    /// - Panics if `index` is out of bounds.
    fn __get_mut(&mut self, index: usize) -> &mut T {
        assert!(index <= self.__len(), "Index out of bounds");
        unsafe { &mut *self.__ptr().as_ptr().add(index) }
    }
}
