use std::{
    alloc::{self, Layout},
    mem,
    ptr::NonNull,
};

trait Ptr<T> {
    fn __get_ptr(&self) -> NonNull<T>;
    fn __set_ptr(&mut self, ptr: NonNull<T>);
}

trait Len {
    fn __get_len(&self) -> usize;
    fn __set_len(&mut self, len: usize);
}

trait Cap {
    fn __get_cap(&self) -> usize;
    fn __set_cap(&mut self, len: usize);
}

trait Grow<T>: Ptr<T> + Cap + GrowManual<T> {
    fn __grow(&mut self) {
        // Doubles the cap of the underlying ptr.
        // __grow_manually adds the parameter to the current cap
        self.__grow_manually(self.__get_cap());
    }
}
trait GrowManual<T>: Ptr<T> + Cap {
    fn __grow_manually(&mut self, size_to_add: usize) {
        assert!(mem::size_of::<T>() != 0, "capacity overflow");

        let (new_cap, new_layout) = if self.__get_cap() == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = self.__get_cap() + size_to_add;
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation to large"
        );

        let new_ptr = if self.__get_cap() == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.__get_cap()).unwrap();
            let old_ptr = self.__get_ptr().as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        self.__set_ptr(match NonNull::new(new_ptr as *mut T) {
            Some(ptr) => ptr,
            None => alloc::handle_alloc_error(new_layout),
        });

        self.__set_cap(new_cap);
    }
}

trait Shrink<T>: Ptr<T> + Cap + Len {}
trait ShrinkManualSafe<T>: Ptr<T> + Cap + Len {}
trait ShrinkManual<T>: Ptr<T> + Cap {}
