use std::{
    alloc::{self, Layout},
    marker::PhantomData,
    mem,
    ops::{Deref, DerefMut},
    ptr::{self, NonNull},
    slice,
};

mod normal;
//TODO: Maybe have the states in a different mod?

pub struct Normal; // Grows
pub struct Dynamic; // Grows and shrinks
pub struct Locked; // Can't push/pop
pub struct Fixed; // Only pop
pub struct Tight; // Always the EXACT size in memory like its elements. Pushing grows by one and
                  // popping shrinks by one.
pub struct Manual; // Growing/Shrinking has to be done manually

//TODO: Impl this for all states. Or make a macro to do this?
pub trait DefaultIter {} // If the state implements this the default iter behaviour gets applied

pub struct Sector<'a, T, State> {
    buf: RawSec<T>,
    len: usize,
    _state: PhantomData<(State, &'a T)>,
}

impl<'a, T, S> Sector<'a, T, S> {
    pub fn new<State>() -> Sector<'a, T, State> {
        Sector {
            buf: RawSec::new(),
            len: 0,
            _state: PhantomData,
        }
    }
}

impl<T, State> Drop for Sector<'_, T, State> {
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

impl<T, State> Deref for Sector<'_, T, State> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.buf.ptr.as_ptr(), self.len) }
    }
}

impl<T, State> DerefMut for Sector<'_, T, State> {
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
            } else if slice.len() == 0 {
                slice.as_ptr()
            } else {
                slice.as_ptr().add(slice.len())
            },
        }
    }
}

impl<'a, T> RawSec<T> {
    fn new() -> Self {
        let cap = if mem::size_of::<T>() == 0 { !0 } else { 0 };
        RawSec {
            ptr: NonNull::dangling(),
            cap,
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
                self.end = self.end.offset(-1);
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

impl<T, State: DefaultIter> IntoIterator for Sector<'_, T, State> {
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

impl<'a, T, State: DefaultIter> Sector<'a, T, State> {
    pub fn drain(&mut self) -> Drain<'a, T> {
        let iter = unsafe { RawIter::new(&self) };
        // Sets the len to 0 to make sure the underlying sector does not get used after free
        self.len = 0;

        Drain {
            iter,
            vec: PhantomData,
        }
    }
}

pub struct Drain<'a, T: 'a> {
    vec: PhantomData<&'a mut Vec<T>>,
    iter: RawIter<T>,
}

//TODO: Look into lifetimes warning
impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for Drain<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, T> Drop for Drain<'a, T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}
