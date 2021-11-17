//! ## Pointers
//! - `*const`
//! - `*mut`
//! - `NonNull`
//!


pub struct Box<T> {
    ptr: std::ptr::NonNull<T>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Box<T> {
    pub fn new(val: T) -> Self {
        let ptr = std::boxed::Box::into_raw(std::boxed::Box::new(val));
        let ptr = unsafe { std::ptr::NonNull::new_unchecked(ptr) };

        Self { ptr, _marker: std::marker::PhantomData }
    }
}

impl<T> std::ops::Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> std::ops::DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T> Drop for Box<T> {
    fn drop(&mut self) {
        let _ = unsafe { std::boxed::Box::from_raw(self.ptr.as_ptr()) };
    }
}
