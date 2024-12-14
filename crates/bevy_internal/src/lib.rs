#![deny(warnings)]
#![feature(concat_bytes)]
#![feature(extern_types)]
#![feature(fn_ptr_trait)]
#![feature(ptr_metadata)]
#![feature(slice_from_ptr_range)]
#![feature(tuple_trait)]

use region::Protection;
use std::{mem, ptr};

pub use crate::library::Library;
pub use crate::map::{Map, Maps, Permissions};
pub use crate::traits::{FnPtr, Ptr};
pub use iced_x86;

mod library;
mod map;
mod traits;

#[doc(hidden)]
pub mod app;

#[doc(hidden)]
pub mod macros;

pub mod assembly;

/// Moves `src` into the pointed `dst`, returning the previous `dst` value.
///
/// # Safety
///
/// See [`replace`](std::ptr::replace).
pub unsafe fn replace<T>(dst: *mut T, src: T) -> T {
    let permissions = Maps::permissions_of(dst as *const u8);

    if permissions.contains(Permissions::WRITE) {
        unsafe { ptr::replace(dst, src) }
    } else {
        let _guard = unsafe {
            region::protect_with_handle(
                dst as *const u8,
                mem::size_of::<T>(),
                Protection::READ_WRITE_EXECUTE,
            )
        };

        unsafe { ptr::replace(dst, src) }
    }
}
