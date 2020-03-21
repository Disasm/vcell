//! Just like [`Cell`] but with [volatile] read / write operations
//!
//! [`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
//! [volatile]: https://doc.rust-lang.org/std/ptr/fn.read_volatile.html

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate avatar_common;

use core::cell::UnsafeCell;
use core::ptr;
use avatar_common::StaticMemoryInterface;
use core::sync::atomic::{AtomicPtr, Ordering};
use core::ptr::null_mut;

static INTERFACE: AtomicPtr<StaticMemoryInterface> = AtomicPtr::new(null_mut());

fn memory_interface() -> Option<&'static mut StaticMemoryInterface> {
    let ptr = INTERFACE.load(Ordering::SeqCst);
    if ptr.is_null() {
        return None
    }
    unsafe {
        Some(&mut *ptr)
    }
}

/// Overrides the default memory interface
pub fn set_memory_interface(interface: &'static mut StaticMemoryInterface) {
    INTERFACE.store(interface, Ordering::SeqCst);
}

/// Just like [`Cell`] but with [volatile] read / write operations
///
/// [`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
/// [volatile]: https://doc.rust-lang.org/std/ptr/fn.read_volatile.html
pub struct VolatileCell<T> {
    value: UnsafeCell<T>,
}

impl<T> VolatileCell<T> {
    /// Creates a new `VolatileCell` containing the given value
    pub const fn new(value: T) -> Self {
        VolatileCell { value: UnsafeCell::new(value) }
    }

    /// Returns a copy of the contained value
    #[inline(always)]
    pub fn get(&self) -> T
        where T: Copy
    {
        if let Some(mem) = memory_interface() {
            let address = self.value.get() as usize as u32;
            mem.read(address)
        } else {
            unsafe { ptr::read_volatile(self.value.get()) }
        }
    }

    /// Sets the contained value
    #[inline(always)]
    pub fn set(&self, value: T)
        where T: Copy
    {
        if let Some(mem) = memory_interface() {
            let address = self.value.get() as usize as u32;
            mem.write(address, value)
        } else {
            unsafe { ptr::write_volatile(self.value.get(), value) }
        }
    }

    /// Returns a raw pointer to the underlying data in the cell
    #[inline(always)]
    pub fn as_ptr(&self) -> *mut T {
        self.value.get()
    }
}


// NOTE implicit because of `UnsafeCell`
// unsafe impl<T> !Sync for VolatileCell<T> {}
