use std::ops::Deref;

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use sp__s_borrower::sBorrower;

pub struct sOwner<T> {
    value: T,
    ref_count: AtomicUsize,
}

impl<T> sOwner<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            ref_count: AtomicUsize::new(0),
        }
    }

    pub fn as_ref(&self) -> &T {
        &self.value
    }

    pub fn borrow(&self) -> sBorrower<T> {
        self.ref_count.fetch_add(1, Ordering::Acquire);

        let value_ptr = &self.value as *const T;
        let ref_count_ptr = &self.ref_count as *const AtomicUsize;

        sBorrower::new(value_ptr, ref_count_ptr)
    }
}

impl<'a, T> sOwner<&'a T> {
    pub fn borrow_deref(&'a self) -> sBorrower<T> {
        self.ref_count.fetch_add(1, Ordering::Acquire);

        let value_ptr = self.value as *const T;
        let ref_count_ptr = &self.ref_count as *const AtomicUsize;

        sBorrower::new(value_ptr, ref_count_ptr)
    }
}

impl<T> Deref for sOwner<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> Drop for sOwner<T> {
    fn drop(&mut self) {
        if self.ref_count.load(Ordering::Relaxed) > 0 {
            panic!("Attempted to drop sOwner before all of its associated sBorrowers were dropped.");
        }
    }
}
