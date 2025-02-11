use std::ptr::NonNull;

/// **Trait `Ptr<T>`**
///
/// Represents an interface for handling raw pointers for collection elements.
///
/// - [`__ptr()`] should return the internal pointer.
/// - [`__ptr_set(ptr)`] should set the internal pointer to a new location.
///
/// *Has to implement the `Drop` trait to ensure deallocating the underlying pointer*
#[allow(drop_bounds)]
pub trait Ptr<T>: Drop {
    /// Returns the internal pointer.
    fn __ptr(&self) -> NonNull<T>;

    /// Sets the internal pointer to a new location.
    ///
    /// # Arguments
    ///
    /// * `new_ptr` - The new non-null pointer to replace the existing pointer.
    fn __ptr_set(&mut self, new_ptr: NonNull<T>);
}
