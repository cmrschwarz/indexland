use std::{
    mem::ManuallyDrop,
    panic::{RefUnwindSafe, UnwindSafe},
};

/// An alias type for `T` that
/// - cannot be constructed (safely)
/// - has the same size, align, **and niches** as T
/// - has no Drop impl
/// - implementis `Send`, `Sync`, `Unpin`, 'Clone', `UnwindSafe` and
///   `RefUnwindSafe` regardless of `T`
///
/// This type is mainly intended for usage with `transmute_vec`
#[repr(transparent)]
pub struct PhantomSlot<T> {
    _internal: ManuallyDrop<T>,
}

unsafe impl<T> Send for PhantomSlot<T> {}
unsafe impl<T> Sync for PhantomSlot<T> {}
impl<T> Unpin for PhantomSlot<T> {}
impl<T> RefUnwindSafe for PhantomSlot<T> {}
impl<T> UnwindSafe for PhantomSlot<T> {}

impl<T> Clone for PhantomSlot<T> {
    fn clone(&self) -> Self {
        panic!("attempted to clone phantom slot")
    }
}
