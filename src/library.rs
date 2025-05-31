use std::ffi::{self, CStr, OsStr};
use std::marker::PhantomData;
use std::mem;
use std::path::Path;
use std::sync::Arc;

pub struct Library {
    library: libloading::Library,
}

pub struct Item<T> {
    library: Arc<Library>,
    symbol: libloading::Symbol<'static, *mut ffi::c_void>,
    _phantom: PhantomData<T>,
}

impl Library {
    pub fn open(path: impl AsRef<Path>) -> Result<Arc<Self>, String> {
        let path = path.as_ref();

        match unsafe { libloading::Library::new(path) } {
            Ok(library) => Ok(Arc::new(Self { library })),
            Err(error) => Err(format!(
                "failed to load library `{}`: {error}",
                path.display()
            )),
        }
    }

    pub fn get<T>(self: &Arc<Self>, item: &CStr) -> Result<Item<T>, String> {
        match unsafe {
            self.library
                .get::<*mut ffi::c_void>(item.to_bytes_with_nul())
        } {
            Ok(symbol) => Ok(Item {
                library: Arc::clone(self),
                symbol: unsafe { extend_symbol_lifetime(symbol) },
                _phantom: PhantomData,
            }),
            Err(error) => Err(format!("failed to load symbol `{}`: {error}", unsafe {
                OsStr::from_encoded_bytes_unchecked(item.to_bytes()).display()
            })),
        }
    }
}

// Erase the provided `libloading::Symbol`'s lifetime.
unsafe fn extend_symbol_lifetime<T>(
    symbol: libloading::Symbol<'_, T>,
) -> libloading::Symbol<'static, T> {
    unsafe { mem::transmute(symbol) }
}

impl<T> Item<T> {
    fn as_raw_ptr(&self) -> *mut ffi::c_void {
        unsafe { self.symbol.clone().try_as_raw_ptr().unwrap() }
    }

    pub fn as_ptr(&self) -> *const T {
        self.as_raw_ptr().cast_const().cast()
    }
}
