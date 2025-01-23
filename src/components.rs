use std::{
    alloc::{self, Layout},
    mem,
    ptr::{self, NonNull},
};

/// **Trait `Ptr<T>`**
///
/// Represents an interface for handling raw pointers for collection elements.
///
/// - [`__ptr()`] should return the internal pointer.
/// - [`__ptr_set(ptr)`] should set the internal pointer to a new location.
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

/// **Trait `Cap`**
///
/// Provides an interface to track capacity of allocated memory.
///
/// - `__cap()` - Gets the current capacity.
/// - `__cap_set(cap)` - Sets the capacity.
pub trait Cap {
    /// Returns the current capacity.
    fn __cap(&self) -> usize;

    /// Sets the capacity to a new value.
    ///
    /// # Arguments
    ///
    /// * `new_cap` - The new capacity.
    fn __cap_set(&mut self, new_cap: usize);
}

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

        let (new_cap, new_layout) = if self.__cap() == 0 {
            (1, Layout::array::<T>(1).unwrap())
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
    /// This function should be called regardless of whether memory actually needs
    /// to grow or not, as it handles the growth decision internally.
    ///
    /// # Safety
    ///
    /// This trait hevily relies on the '__grow()' implementation, therefore if the implementation of
    /// the mentioned function is wrong it will cause Undefined Behavior.
    ///
    /// <div class="warning">
    /// **Warning:** Incorrect implementation will cause undefined behavior.
    /// </div>
    unsafe fn __grow(&mut self);
}

/// **Trait `Shrink<T>`**
///
/// Manages reduction of allocated memory by deallocating unused space.
///
/// # Safety
///
/// This trait hevily relies on the '__shrink()' implementation, therefore if the implementation of
/// the mentioned function is wrong it will cause Undefined Behavior.
///
/// - `__shrink_manually` - Reduces the capacity by a specified amount.
/// - `__shrink` - Placeholder for automatic shrink handling.
pub unsafe trait Shrink<T>: Cap + Ptr<T> {
    /// Manually reduces the allocated memory by a specified number of elements.
    ///
    /// # Arguments
    ///
    /// * `len_to_sub` - The number of elements to reduce from the current capacity.
    ///
    /// # Panics
    ///
    /// - Panics if `len_to_sub` exceeds the current capacity.
    fn __shrink_manually(&mut self, len_to_sub: usize) {
        assert!(mem::size_of::<T>() != 0, "Capacity overflow");
        assert!(len_to_sub <= self.__cap(), "Capacity underflow");

        let new_cap = self.__cap() - len_to_sub;
        let new_layout = Layout::array::<T>(new_cap).unwrap();

        let new_ptr = if new_layout.size() > 0 {
            let old_layout = Layout::array::<T>(self.__cap()).unwrap();
            let old_ptr = self.__ptr().as_ptr() as *mut u8;

            let new_u8_ptr = unsafe { alloc::realloc(old_ptr, old_layout, new_cap) };

            match NonNull::new(new_u8_ptr as *mut T) {
                Some(ptr) => ptr,
                None => alloc::handle_alloc_error(new_layout),
            }
        } else {
            NonNull::dangling()
        };

        self.__ptr_set(new_ptr);
        self.__cap_set(new_cap);
    }

    /// Automatically grows the memory when needed.
    ///
    /// This function should be called regardless of whether memory actually needs
    /// to grow or not, as it handles the growth decision internally.
    ///
    /// # Safety
    ///
    /// This trait hevily relies on the '__shrink()' implementation, therefore if the implementation of
    /// the mentioned function is wrong it will cause Undefined Behavior.
    ///
    /// <div class="warning">
    /// **Warning:** Incorrect implementation will cause undefined behavior.
    /// </div>
    unsafe fn __shrink(&mut self);
}

/// **Trait `Resize<T>`**
///
/// Resizes the allocation to a specified capacity directly.
///
/// - `__resize` - Changes capacity to a given number of elements.
pub trait Resize<T>: Cap + Ptr<T> {
    /// Resizes the allocation to the specified capacity.
    ///
    /// # Arguments
    ///
    /// * `new_cap` - The desired new capacity.
    ///
    /// # Panics
    ///
    /// - Panics if the allocation size exceeds `isize::MAX`.
    fn __resize(&mut self, new_cap: usize) {
        let new_layout = Layout::array::<T>(new_cap).unwrap();
        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Capacity overflow"
        );

        let new_ptr = if self.__cap() == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.__cap()).unwrap();
            let old_ptr = self.__ptr().as_ptr() as *mut u8;

            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        let new_ptr = match NonNull::new(new_ptr as *mut T) {
            Some(ptr) => ptr,
            None => alloc::handle_alloc_error(new_layout),
        };

        self.__cap_set(new_cap);
        self.__ptr_set(new_ptr);
    }
}

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
        // The grow implementation should handle whether or not to grow the underlying pointer
        unsafe { self.__grow() };

        assert!(len < self.__cap(), "Incorrect Grow implementation");

        unsafe { ptr::write(self.__ptr().as_ptr().add(len), elem) }
        self.__len_set(len + 1);
    }
}

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
        // The grow implementation should handle whether or not to grow the underlying pointer
        unsafe { self.__grow() };

        assert!(len < self.__cap(), "Incorrect Grow implementation");

        unsafe {
            ptr::copy(
                self.__ptr().as_ptr().add(index),
                self.__ptr().as_ptr().add(index + 1),
                len - index,
            );

            ptr::write(self.__ptr().as_ptr().add(index), elem);
        }

        self.__len_set(len + 1);
    }
}

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
