use std::{
    alloc::{self, Layout},
    marker::PhantomData,
    mem::{self, ManuallyDrop},
    ops::{Deref, DerefMut},
    ptr::{self, NonNull},
    slice,
};

mod normal;

pub struct Normal; // Grows
pub struct Dynamic; // Grows and shrinks
pub struct Locked; // Can't push/pop
pub struct Fixed; // Only pop
pub struct Tight; // Always the EXACT size in memory like its elements. Pushing grows by one and
                  // popping shrinks by one.
pub struct Manual; // Growing/Shrinking has to be done manually

//TODO IMPL FOR STATES
pub trait DefaultIter {} // If the state implements this the default iter behaviour gets applied

struct RawSec<T> {
    ptr: NonNull<T>,
    cap: usize,
}

struct RawIter<T> {
    start: *const T,
    end: *const T,
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
        if self.cap != 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe { alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout) }
        }
    }
}

pub struct Sector<'a, T, State> {
    buf: RawSec<T>,
    len: usize,
    _state: PhantomData<(State, &'a T)>,
}

impl<'a, T, State> Sector<'a, T, State> {
    pub fn new() -> Sector<'a, T, Normal> {
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

pub struct IntoIter<T> {
    _buf: RawSec<T>,
    start: *const T,
    end: *const T,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let result = ptr::read(self.start);
                self.start = self.start.offset(1);
                Some(result)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.end as usize - self.start as usize) / size_of::<T>();
        (size, Some(size))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.buf.cap == 0 {
            None
        } else {
            unsafe {
                self.end = self.end.offset(-1);
                Some(ptr::read(self.end))
            }
        }
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
        let buf = unsafe { ptr::read(&self.buf) };
        let len = self.len;
        mem::forget(self);
        IntoIter {
            start: buf.ptr.as_ptr(),
            end: if buf.cap == 0 {
                buf.ptr.as_ptr()
            } else {
                unsafe { buf.ptr.as_ptr().add(len) }
            },
            _buf: buf,
        }
    }
}
