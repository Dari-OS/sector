use std::{
    alloc::{self, Layout, LayoutError},
    marker::PhantomData,
    mem,
    ops::{Deref, DerefMut},
    ptr::{self, NonNull},
    slice,
};

pub struct Sector<State, T> {
    buf: RawSec<T>,
    len: usize,
    _state: PhantomData<State>,
}

impl<State, T> Sector<State, T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Sector<State, T> {
        Sector {
            buf: RawSec::new(),
            len: 0,
            _state: PhantomData,
        }
    }

    pub fn with_capacity(capacity: usize) -> Sector<State, T> {
        Sector {
            buf: RawSec::with_capacity(capacity),
            len: 0,
            _state: PhantomData,
        }
    }

    pub fn try_with_capacity(capacity: usize) -> Result<Sector<State, T>, LayoutError> {
        Ok(Sector {
            buf: RawSec::try_with_capacity(capacity)?,
            len: 0,
            _state: PhantomData,
        })
    }

    //  TODO: DOC on how unsafe using this is. Can point to NULL
    #[allow(dead_code)]
    pub(crate) unsafe fn get_ptr(&self) -> NonNull<T> {
        self.buf.ptr
    }

    //  TODO: DOC on how unsafe using this is. Can point to NULL
    // Changing it can cause side-effects (UB)
    #[allow(dead_code)]
    pub(crate) unsafe fn get_ptr_mut(&mut self) -> NonNull<T> {
        self.buf.ptr
    }

    //   TODO: DOC on how unsafe using this is. it is. REALLY UNSAFE!
    #[allow(dead_code)]
    pub(crate) unsafe fn set_ptr(&mut self, new_ptr: NonNull<T>) {
        self.buf.ptr = new_ptr;
    }

    #[allow(dead_code)]
    pub(crate) fn get_cap(&self) -> usize {
        self.buf.cap
    }

    //  TODO: DOC on how unsafe using this is. it is. REALLY UNSAFE!
    #[allow(dead_code)]
    pub(crate) unsafe fn set_cap(&mut self, new_cap: usize) {
        self.buf.cap = new_cap;
    }

    #[allow(dead_code)]
    pub(crate) fn get_len(&self) -> usize {
        self.len
    }

    //  TODO: DOC on how unsafe using this is. it is. REALLY UNSAFE!
    #[allow(dead_code)]
    pub(crate) unsafe fn set_len(&mut self, new_len: usize) {
        self.len = new_len;
    }
}

impl<State, T> Drop for Sector<State, T> {
    fn drop(&mut self) {
        if self.len > 0 && mem::size_of::<T>() != 0 {
            for i in 0..self.len {
                unsafe {
                    let ptr = self.buf.ptr.as_ptr().add(i);
                    ptr::drop_in_place(ptr);
                }
            }
        }
    }
}

impl<State, T> Deref for Sector<State, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.buf.ptr.as_ptr(), self.len) }
    }
}

impl<State, T> DerefMut for Sector<State, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { slice::from_raw_parts_mut(self.buf.ptr.as_ptr(), self.len) }
    }
}

struct RawSec<T> {
    ptr: NonNull<T>,
    cap: usize,
}

struct RawIter<T> {
    start: *const T,
    end: *const T,
}

impl<T> RawIter<T> {
    unsafe fn new(slice: &[T]) -> Self {
        RawIter {
            start: slice.as_ptr(),
            end: if size_of::<T>() == 0 {
                ((slice.as_ptr() as usize) + slice.len()) as *const _
            } else if slice.is_empty() {
                slice.as_ptr()
            } else {
                slice.as_ptr().add(slice.len())
            },
        }
    }
}

impl<T> RawSec<T> {
    fn new() -> Self {
        let (ptr, cap) = Self::create_ptr(None).unwrap();
        RawSec { ptr, cap }
    }

    fn with_capacity(capacity: usize) -> Self {
        let (ptr, cap) = Self::create_ptr(Some(capacity))
            .unwrap_or_else(|_| panic!("The given capacity {capacity} overflows the layout"));
        RawSec { ptr, cap }
    }

    #[allow(dead_code)]
    fn try_with_capacity(capacity: usize) -> Result<Self, LayoutError> {
        let (ptr, cap) = Self::create_ptr(Some(capacity))?;
        Ok(RawSec { ptr, cap })
    }

    /// Creates a new (_allocated_) pointer and capacity with the correct size
    ///
    /// # Returns
    ///
    /// `(NonNull<T>, usize)` ~ Ptr to the allocated pointer (if no ZST) and capacity (May not be
    /// the original one if the type is ZST)
    ///
    /// `LayoutError` ~ On arithmetic overflow or when the total size would exceed
    /// __isize::MAX__
    ///
    /// # Aborts
    ///
    /// Aborts if a allocation error occures
    // TODO: Look into returning `TryReserverError`.
    // Currently not possible because of the unstable status of `TryReserverErrorKind`
    // See: https://github.com/rust-lang/rust/issues/48043
    fn create_ptr(initial_capacity: Option<usize>) -> Result<(NonNull<T>, usize), LayoutError> {
        let capacity = initial_capacity.unwrap_or_default();
        if size_of::<T>() == 0 {
            return Ok((NonNull::dangling(), !0));
        }
        if capacity == 0 {
            return Ok((NonNull::dangling(), 0));
        }
        let layout = Layout::array::<T>(capacity)?;
        let ptr = unsafe { NonNull::new(alloc::alloc(layout) as *mut T) };
        match ptr {
            Some(ptr) => Ok((ptr, capacity)),
            None => alloc::handle_alloc_error(layout),
        }
    }
}

impl<T> Drop for RawSec<T> {
    fn drop(&mut self) {
        if self.cap != 0 && size_of::<T>() != 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe { alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout) }
        }
    }
}

pub struct IntoIter<T> {
    _buf: RawSec<T>,
    iter: RawIter<T>,
}

impl<T> Iterator for RawIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let result = ptr::read(self.start);
                self.start = if size_of::<T>() == 0 {
                    (self.start as usize + 1) as *const _
                } else {
                    self.start.offset(1)
                };
                Some(result)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.end as usize - self.start as usize) / size_of::<T>();
        (size, Some(size))
    }
}

impl<T> DoubleEndedIterator for RawIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                self.end = if size_of::<T>() == 0 {
                    (self.end as usize - 1) as *const _
                } else {
                    self.end.offset(-1)
                };

                Some(ptr::read(self.end))
            }
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

impl<State: crate::components::DefaultIter, T> IntoIterator for Sector<State, T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            let iter = RawIter::new(&self);
            let buf = ptr::read(&self.buf);

            mem::forget(self);
            IntoIter { iter, _buf: buf }
        }
    }
}

impl<State: crate::components::DefaultDrain, T> Sector<State, T> {
    pub fn drain(&mut self) -> Drain<'_, T> {
        let iter = unsafe { RawIter::new(self) };
        // Sets the len to 0 to make sure the underlying sector does not get used after free
        self.len = 0;

        Drain {
            iter,
            sec: PhantomData,
        }
    }
}

pub struct Drain<'a, T: 'a> {
    sec: PhantomData<&'a mut Sector<(), T>>,
    iter: RawIter<T>,
}

impl<T> Iterator for Drain<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for Drain<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<T> Drop for Drain<'_, T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}
