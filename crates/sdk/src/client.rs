use crate::{vtable_validate, ClientMode};
use cake::ffi::VTablePad;
use core::{mem, ptr};

pub use class::Class;
pub use classes::ClassIter;
pub use property::Property;
pub use table::Table;

mod class;
mod classes;
mod property;
mod table;

#[repr(C)]
struct VTable {
    _pad0: VTablePad<8>,
    class_iter: unsafe extern "C" fn(this: *const Client) -> *const Class,
    _pad1: VTablePad<1>,
    hud_process_input: unsafe extern "C" fn(),
    hud_update: unsafe extern "C" fn(),
    _pad2: VTablePad<4>,
    activate_mouse: unsafe extern "C" fn(),
    _pad3: VTablePad<20>,
    frame_stage_notify: unsafe extern "C" fn(this: *const (), frame: i32) -> bool,
    dispatch_user_message: unsafe extern "C" fn(
        this: *const Client,
        message_kind: i32,
        passthrough_flags: i32,
        len: i32,
        data: *const u8,
    ) -> bool,
}

vtable_validate! {
    class_iter => 8,
    hud_process_input => 10,
    hud_update => 11,
    activate_mouse => 16,
    frame_stage_notify => 37,
    dispatch_user_message => 38,
}

/// Client interface.
#[repr(C)]
pub struct Client {
    vtable: &'static VTable,
}

impl Client {
    #[inline]
    pub fn class_iter(&self) -> ClassIter<'_> {
        unsafe {
            let iter = (self.vtable.class_iter)(self).as_ref();

            ClassIter { iter }
        }
    }

    #[inline]
    pub fn dispatch_user_message<'a, D>(
        &self,
        kind: i32,
        flags: i32,
        data: Option<&'a [u8]>,
    ) -> bool {
        const EMPTY: (*const u8, i32) = (ptr::null(), 0);

        let (bytes, len) = data
            .map(|bytes| (bytes.as_ptr(), bytes.len() as i32))
            .unwrap_or(EMPTY);

        unsafe { (self.vtable.dispatch_user_message)(self, kind, flags, len, bytes) }
    }

    #[inline]
    pub fn client_mode(&self) -> *const ClientMode {
        unsafe {
            type ClientModeFn = unsafe extern "C" fn() -> *const ClientMode;

            let hud_process_input = self.vtable.hud_process_input as *const u8;
            let call_client_mode = hud_process_input.byte_add(11);
            let client_mode = elysium_mem::next_abs_addr_ptr::<u8>(call_client_mode)
                .expect("client mode function");

            let client_mode: ClientModeFn = mem::transmute(client_mode);

            client_mode()
        }
    }

    #[inline]
    pub fn create_move_address(&self) -> *const u8 {
        unsafe {
            let client_mode = &*self.client_mode();

            client_mode.create_move_address()
        }
    }

    #[inline]
    pub fn override_view_address(&self) -> *const u8 {
        unsafe {
            let client_mode = &*self.client_mode();

            client_mode.override_view_address()
        }
    }

    #[inline]
    pub fn frame_stage_notify_address(&self) -> *const u8 {
        ptr::addr_of!(self.vtable.frame_stage_notify).cast()
    }

    #[inline]
    pub fn globals(&self) -> *const u8 {
        unsafe {
            let hud_update = self.vtable.hud_update as *const u8;
            let address = hud_update.byte_add(13);
            let globals = elysium_mem::next_abs_addr_ptr::<*const u8>(address)
                .expect("globals structure")
                .read();

            globals
        }
    }

    #[inline]
    pub fn input(&self) -> *const u8 {
        unsafe {
            let activate_mouse = self.vtable.activate_mouse as *const u8;
            let input = elysium_mem::next_abs_addr_ptr::<*const *const u8>(activate_mouse)
                .expect("input structure")
                .read()
                .read();

            input
        }
    }
}
