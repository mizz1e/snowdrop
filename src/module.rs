use super::library::{Item, Library};
use super::x86;
use std::ffi::{CStr, OsStr};
use std::{fmt, slice};

/// The engine version.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Version {
    Source1,
    Source2,
}

impl fmt::Display for Version {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            Self::Source1 => "1",
            Self::Source2 => "2",
        };

        fmt.write_str(string)
    }
}

/// An engine module.
pub struct Module {
    constructor: Item<u8>,
    registry: Option<Item<*const ffi::Interface>>,
    registry_head: *const ffi::Interface,
    version: Version,
}

impl Module {
    /// Open the specified file name.
    pub fn open(file_name: impl AsRef<OsStr>) -> Result<Self, String> {
        let file_name = file_name.as_ref();
        let library = Library::open(file_name)?;
        let constructor = library.get(c"CreateInterface")?;

        if let Ok(registry) = library.get(c"s_pInterfaceRegs") {
            return Ok(Self {
                constructor,
                registry_head: unsafe { *registry.as_ptr() },
                registry: Some(registry),
                version: Version::Source1,
            });
        }

        if let Ok(this) = Self::resolve_registry(constructor) {
            return Ok(this);
        }

        Err(format!(
            "`{}` does not use a supported moduel system",
            file_name.display()
        ))
    }

    fn resolve_registry(constructor: Item<u8>) -> Result<Self, String> {
        let addr = constructor.as_ptr();

        // Resolve `CreateInterface` from the thunk.
        let addr = x86::resolve_relative(
            [],
            [
                0xE9, // jmp <disp>
            ],
            unsafe { slice::from_raw_parts(addr, 5) },
        )
        .map_err(|error| format!("{error}"))?;

        // Resolve `s_pInterfaceRegs` from `mov rbx, [rip + <disp>]`.
        let registry = x86::resolve_relative(
            [
                0x55, // push rbp
                0x48, 0x89, 0xE5, // mov rbp, rsp,
                0x41, 0x55, // push r13
                0x49, 0x89, 0xF5, // mov r13, rsi
                0x41, 0x54, // push r12
                0x53, // push rbx
                0x48, 0x83, 0xEC, 0x08, // sub rsp, 8
            ],
            [
                0x48, 0x8B, 0x1D, // mov rbx, [rip + <disp>]
            ],
            unsafe { slice::from_raw_parts(addr, 23) },
        )
        .map_err(|error| format!("{error}"))?;

        Ok(Self {
            constructor,
            registry_head: unsafe { *registry.cast() },
            registry: None,
            version: Version::Source2,
        })
    }

    pub fn interfaces(&self) -> impl Iterator<Item = Interface> {
        let iter = ffi::Iter {
            head: self.registry_head,
        };

        iter.map(|interface| {
            let name = unsafe { CStr::from_ptr(interface.name) };

            Interface { name }
        })
    }

    pub fn version(&self) -> Version {
        self.version
    }
}

#[derive(Debug)]
pub struct Interface<'interface> {
    pub name: &'interface CStr,
}

mod ffi {
    use std::ffi;

    #[derive(Debug)]
    #[repr(C)]
    pub struct Interface {
        constructor: *const ffi::c_void,
        pub(super) name: *const ffi::c_char,
        next: *const Self,
    }

    pub(super) struct Iter {
        pub(super) head: *const Interface,
    }

    impl Iterator for Iter {
        type Item = Interface;

        fn next(&mut self) -> Option<Self::Item> {
            if self.head.is_null() {
                None
            } else {
                let next = unsafe { self.head.read() };

                self.head = next.next;

                Some(next)
            }
        }
    }
}
