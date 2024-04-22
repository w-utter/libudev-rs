use ::handle::Handle;
use std::ptr::NonNull;

/// A libudev context. Contexts may not be sent or shared between threads. The `libudev(3)` manpage
/// says:
///
/// > All functions require a libudev context to operate. This context can be create via
/// > udev_new(3). It is used to track library state and link objects together. No global state is
/// > used by libudev, everything is always linked to a udev context. Furthermore, multiple
/// > different udev contexts can be used in parallel by multiple threads. However, a single
/// > context must not be accessed by multiple threads in parallel.
///
/// In Rust, that means that `Context` is `!Send` and `!Sync`. This means a `Context` must be
/// created in the thread where it will be used. Several contexts can exist in separate threads,
/// but they can not be sent between threads.
///
/// Other types in this library (`Device`, `Enumerator`, `Monitor`, etc.) share a reference to a
/// context, which means that these types must also be `!Send` and `!Sync`.
pub struct Context {
    udev: NonNull<::ffi::udev>,
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Clone for Context {
    /// Increments reference count of `libudev` context.
    fn clone(&self) -> Self {
        Context {
            //SAFETY: if self contains a valid pointer, then a clone of the pointer is also valid.
            udev: unsafe { NonNull::new_unchecked(::ffi::udev_ref(self.udev.as_ptr())) },
        }
    }
}

impl Drop for Context {
    /// Decrements reference count of `libudev` context.
    fn drop(&mut self) {
        unsafe {
            ::ffi::udev_unref(self.udev.as_ptr());
        }
    }
}

#[doc(hidden)]
impl Handle<::ffi::udev> for Context {
    fn as_ptr(&self) -> *mut ::ffi::udev {
        unsafe {self.udev.as_ptr() }
    }
}

impl Context {
    /// Creates a new context.
    pub fn new() -> ::Result<Self> {
        //SAFETY: the try_alloc will catch any null ptrs
        let udev = unsafe { NonNull::new_unchecked(try_alloc!(::ffi::udev_new()))};
        Ok(Context {
            udev,
        })
    }
}
