/// **Trait `Cap`**
///
/// Provides an interface to track the capacity of allocated memory.
///
/// # Methods
/// - `__cap()` - Retrieves the current capacity.
/// - `__cap_set(cap)` - Updates the capacity.
pub trait Cap {
    /// Returns the current capacity.
    fn __cap(&self) -> usize;

    /// Sets the capacity to a new value.
    ///
    /// # Arguments
    /// * `new_cap` - The new capacity.
    fn __cap_set(&mut self, new_cap: usize);
}
