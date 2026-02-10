use std::ops::Deref;

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

pub struct sBorrower<T> {
    value_ptr: *const T,
    ref_count_ptr: *const AtomicUsize,
}

impl<T> sBorrower<T> {
    pub fn new(value_ptr: *const T, ref_count_ptr: *const AtomicUsize) -> Self {
        Self {
            value_ptr,
            ref_count_ptr,
        }
    }

    pub fn as_ref(&self) -> &T {
        unsafe {
            self.value_ptr.as_ref().expect("Attempted to access a sBorrower's contained value after its associated sOwner was dropped.")
        }
    }
}

impl<T> Clone for sBorrower<T> {
    fn clone(&self) -> Self {
        let ref_count = unsafe {
            self.ref_count_ptr.as_ref()
        }.expect("Attempted to clone an sBorrower after its associated sOwner was dropped.");

        ref_count.fetch_add(1, Ordering::SeqCst);

        let value_ptr = self.value_ptr;
        let ref_count_ptr = self.ref_count_ptr;

        sBorrower::new(value_ptr, ref_count_ptr)
    }
}

impl<T> Deref for sBorrower<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> Drop for sBorrower<T> {
    fn drop(&mut self) {
        unsafe {
            self.ref_count_ptr.as_ref().unwrap().fetch_sub(1, Ordering::Release);
        }
    }
}

unsafe impl<T: Send> Send for sBorrower<T> {}

unsafe impl<T: Sync> Sync for sBorrower<T> {}
