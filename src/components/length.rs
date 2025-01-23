/// **Trait `Len`**
///
/// Interface for tracking the length or "used portion" of the allocated memory.
///
/// - `__len()` - Gets the current number of elements.
/// - `__len_set()` - Sets the number of elements.
pub trait Len {
    /// Returns the current length, representing the number of elements in use.
    fn __len(&self) -> usize;

    /// Sets the length to a new value.
    ///
    /// # Arguments
    ///
    /// * `new_len` - The updated length.
    fn __len_set(&mut self, new_len: usize);
}
