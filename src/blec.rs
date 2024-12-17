use std::{
    alloc::{self, Layout},
    marker::PhantomData,
    mem::{self, ManuallyDrop},
    ops::{Deref, DerefMut},
    ptr::{self, NonNull},
    slice,
};

mod normal;
use crate::components::Ptr;

pub struct Normal; // Grows
pub struct Dynamic; // Grows and shrinks
pub struct Locked; // Can't push/pop
pub struct Fixed; // Only pop
pub struct Tight; // Always the EXACT size in memory like its elements. Pushing grows by one and
                  // popping shrinks by one.
pub struct Manual; // Growing/Shrinking has to be done manually

//TODO IMPL FOR STATES
pub trait DefaultIter {} // If the state implements this the default iter behaviour gets applied

pub struct Blec<'a, T: 'a, State> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
    phantom: PhantomData<(State, &'a T)>,
}

impl<'a, T, State> Blec<'a, T, State> {
    pub fn new() -> Blec<'a, T, Normal> {
        let cap = if mem::size_of::<T>() == 0 { !0 } else { 0 };

        Blec {
            ptr: NonNull::dangling(),
            cap,
            len: 0,
            phantom: PhantomData::<(Normal, &'a T)>,
        }
    }
}

impl<T, State> Drop for Blec<'_, T, State> {
    fn drop(&mut self) {
        if self.cap != 0 {
            if self.len > 0 && mem::size_of::<T>() != 0 {
                for i in 0..self.len {
                    unsafe {
                        let ptr = self.ptr.as_ptr().add(i);
                        ptr::drop_in_place(ptr);
                    }
                }
            }
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<T, Unlocked> Deref for Blec<'_, T, Unlocked> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T, Unlocked> DerefMut for Blec<'_, T, Unlocked> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

pub struct IntoIter<T> {
    buf: NonNull<T>,
    cap: usize,
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
        if self.cap == 0 {
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
        if self.cap != 0 {
            for _ in &mut *self {}
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.buf.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<T, State: DefaultIter> IntoIterator for Blec<'_, T, State> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let blec = ManuallyDrop::new(self);

        IntoIter {
            buf: blec.ptr,
            cap: blec.cap,
            start: blec.ptr.as_ptr(),
            end: if blec.cap == 0 {
                blec.ptr.as_ptr()
            } else {
                unsafe { blec.ptr.as_ptr().add(blec.len) }
            },
        }
    }
}

//impl<'a, T, Normal> Blec<'a, T, Normal> {
//    fn grow(&mut self) {}
//
//    fn push(&mut self, element: T) {
//        if self.len == self.cap {
//            self.grow();
//        }
//
//        unsafe {
//            ptr::write(self.ptr.as_ptr().add(self.len), element);
//        }
//
//        self.len += 1;
//    }
//
//    fn pop(&mut self) -> Option<T> {
//        if self.len == 0 {
//            None
//        } else {
//            self.len -= 1;
//
//            unsafe { Some(ptr::read(self.ptr.as_ptr().add(self.len))) }
//        }
//    }
//
//    fn insert(&mut self, index: usize, element: T) {
//        assert!(index <= self.len, "index out of bounds");
//
//        if self.len == self.cap {
//            self.grow();
//        }
//
//        unsafe {
//            ptr::copy(
//                self.ptr.as_ptr().add(index),
//                self.ptr.as_ptr().add(index + 1),
//                self.len - index,
//            );
//
//            ptr::write(self.ptr.as_ptr().add(index), element);
//        }
//
//        self.len += 1;
//    }
//    fn remove(&mut self, index: usize) -> T {
//        assert!(index < self.len, "index out of bounds");
//        unsafe {
//            self.len -= 1;
//            let result = ptr::read(self.ptr.as_ptr().add(index));
//            ptr::copy(
//                self.ptr.as_ptr().add(index + 1),
//                self.ptr.as_ptr().add(index),
//                self.len - index,
//            );
//            return result;
//        }
//    }
//}
