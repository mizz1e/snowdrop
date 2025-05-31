use super::library::{Item, Library};
use super::x86;
use std::ffi::CStr;
use std::path::Path;
use std::slice;
use std::sync::Arc;

/// Load a Source 1 module.
fn source1(library: &Arc<Library>) -> Result<Module, String> {
    let interface_registry: Item<*const ffi::Interface> = library.get(c"s_pInterfaceRegs")?;

    Ok(Module {
        library: Arc::clone(&library),
        interface_registry: unsafe { *interface_registry.as_ptr() },
    })
}

const CREATE_INTERFACE_THUNK: [u8; 0] = [];

const CREATE_INTERFACE_THUNK_INST: [u8; 1] = [
    0xE9, // jmp <disp>
];

const CREATE_INTERFACE: [u8; 16] = [
    0x55, // push rbp
    0x48, 0x89, 0xE5, // mov rbp, rsp,
    0x41, 0x55, // push r13
    0x49, 0x89, 0xF5, // mov r13, rsi
    0x41, 0x54, // push r12
    0x53, // push rbx
    0x48, 0x83, 0xEC, 0x08, // sub rsp, 8
];

const CREATE_INTERFACE_INST: [u8; 3] = [
    0x48, 0x8B, 0x1D, // mov rbx, [rip + <disp>]
];

/// Load a Source 2 module.
fn source2(library: &Arc<Library>) -> Result<Module, String> {
    let create_interface: Item<u8> = library.get(c"CreateInterface")?;
    let create_interface_addr = create_interface.as_ptr();

    // Resolve thunk.
    let create_interface_addr = x86::resolve_relative(
        CREATE_INTERFACE_THUNK,
        CREATE_INTERFACE_THUNK_INST,
        unsafe { slice::from_raw_parts(create_interface_addr, 5) },
    )
    .map_err(|error| format!("source2 module: {error}"))?;

    // Resolve interface registry.
    let interface_registry =
        x86::resolve_relative(CREATE_INTERFACE, CREATE_INTERFACE_INST, unsafe {
            slice::from_raw_parts(create_interface_addr, 23)
        })
        .map_err(|error| format!("source2 module: {error}"))?;

    Ok(Module {
        library: Arc::clone(&library),
        interface_registry: unsafe { *interface_registry.cast() },
    })
}

/// A Source module.
pub struct Module {
    library: Arc<Library>,
    interface_registry: *const ffi::Interface,
}

impl Module {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, String> {
        let library = Library::open(path)?;

        source1(&library)
            .or_else(|_| source2(&library))
            .map_err(|error| format!("unsupported module system: {error}"))
    }

    pub fn interfaces(&self) -> impl Iterator<Item = Interface> {
        let iter = ffi::Iter {
            next: self.interface_registry,
        };

        iter.map(|interface| {
            let name = unsafe { CStr::from_ptr(interface.name) };

            Interface { name }
        })
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
        pub(super) next: *const Interface,
    }

    impl Iterator for Iter {
        type Item = Interface;

        fn next(&mut self) -> Option<Self::Item> {
            if self.next.is_null() {
                None
            } else {
                let next = unsafe { self.next.read() };

                self.next = next.next;

                Some(next)
            }
        }
    }
}
