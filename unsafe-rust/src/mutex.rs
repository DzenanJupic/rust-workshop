//! ## Interior mutability
//! - UnsafeCell<T>
//!

use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Mutex<T> {
    value: UnsafeCell<T>,
    is_locked: AtomicBool,
}

unsafe impl<T> Sync for Mutex<T> {}

unsafe impl<T> Send for Mutex<T> {}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
    value: &'a mut T,
}

impl<T> Mutex<T> {
    pub fn new(val: T) -> Self {
        Self {
            value: UnsafeCell::new(val),
            is_locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        loop {
            let res = self.is_locked.compare_exchange(
                false,
                true,
                Ordering::Acquire,
                Ordering::Relaxed,
            );
            if let Ok(_) = res { break; }

            std::hint::spin_loop();
        }

        let value = unsafe { &mut *self.value.get() };
        MutexGuard { mutex: self, value }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.is_locked.store(false, Ordering::Release);
    }
}

impl<'a, T> std::ops::Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'a, T> std::ops::DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}
