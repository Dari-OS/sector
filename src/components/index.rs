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
    /// * `Some(&T)` - Reference to the element.
    /// * `None` - If the index is out of bounds
    ///
    fn __get(&self, index: usize) -> Option<&T> {
        if index >= self.__len() {
            return None;
        }
        unsafe { Some(&*self.__ptr().as_ptr().add(index)) }
    }

    /// Retrieves a mutable reference to an element at a specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the element to retrieve.
    ///
    /// # Returns
    ///
    /// * `Some(&mut T)` - Reference to the element.
    /// * `None` - If the index is out of bounds
    ///
    fn __get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.__len() {
            return None;
        }
        unsafe { Some(&mut *self.__ptr().as_ptr().add(index)) }
    }
}
