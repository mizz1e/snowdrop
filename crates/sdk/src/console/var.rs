use crate::vtable_validate;
use cake::ffi::{BytePad, VTablePad};
use core::fmt;
use core::marker::PhantomData;

mod sealed {
    use super::Var;

    pub trait Sealed: Sized {
        fn read(var: &Var<Self>) -> Self;
        fn write(self, var: &mut Var<Self>);
    }

    impl Sealed for f32 {
        #[inline]
        fn read(var: &Var<f32>) -> Self {
            var.read_f32()
        }

        #[inline]
        fn write(self, var: &mut Var<f32>) {
            var.write_f32(self)
        }
    }

    impl Sealed for i32 {
        #[inline]
        fn read(var: &Var<i32>) -> Self {
            var.read_i32()
        }

        #[inline]
        fn write(self, var: &mut Var<i32>) {
            var.write_i32(self)
        }
    }

    impl Sealed for bool {
        #[inline]
        fn read(var: &Var<bool>) -> Self {
            var.read_i32() != 0
        }

        #[inline]
        fn write(self, var: &mut Var<bool>) {
            var.write_i32(self as i32)
        }
    }
}

/// valid types config variables can store
pub trait Kind: sealed::Sealed {}

impl Kind for f32 {}
impl Kind for i32 {}
impl Kind for bool {}

#[repr(C)]
struct VTable {
    _pad0: VTablePad<15>,
    read_f32: unsafe extern "C" fn(this: *const ()) -> f32,
    read_i32: unsafe extern "C" fn(this: *const ()) -> i32,
    _pad1: VTablePad<1>,
    write_f32: unsafe extern "C" fn(this: *mut (), value: f32),
    write_i32: unsafe extern "C" fn(this: *mut (), value: i32),
}

vtable_validate! {
    read_f32 => 15,
    read_i32 => 16,
    write_f32 => 18,
    write_i32 => 19,
}

/// config variable
#[repr(C)]
pub struct Var<T> {
    vtable: *const VTable,
    next: *const (),
    is_registered: bool,
    _pad0: BytePad<7>,
    name: *const u8,
    description: *const u8,
    flags: i32,
    _pad1: BytePad<4>,
    accessor: *const (),
    parent: *const (),
    default_string: *const u8,
    string: *const u8,
    string_len: i32,
    kind: i32,
    integer: i32,
    has_min: bool,
    _pad2: BytePad<3>,
    min_value: f32,
    has_max: bool,
    _pad3: BytePad<3>,
    max_value: f32,
    _pad4: BytePad<4>,
    class: *const (),
    change_callback: Option<unsafe extern "C" fn()>,
    // we do be owning T, tho
    _phantom: PhantomData<T>,
}

impl<T> fmt::Debug for Var<T> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Var").finish_non_exhaustive()
    }
}

impl<T> Var<T> {
    #[inline]
    fn as_ptr(&self) -> *const () {
        self as *const Self as *const ()
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut () {
        self as *mut Self as *mut ()
    }

    #[inline]
    fn read_f32(&self) -> f32 {
        unsafe { ((*self.vtable).read_f32)(self.as_ptr()) }
    }

    #[inline]
    fn write_f32(&mut self, value: f32) {
        unsafe { ((*self.vtable).write_f32)(self.as_mut_ptr(), value) }
    }

    #[inline]
    fn read_i32(&self) -> i32 {
        unsafe { ((*self.vtable).read_i32)(self.as_ptr()) }
    }

    #[inline]
    fn write_i32(&mut self, value: i32) {
        unsafe { ((*self.vtable).write_i32)(self.as_mut_ptr(), value) }
    }
}

impl<T> Var<T>
where
    T: Kind,
{
    /// read the config variable
    #[inline]
    pub fn read(&self) -> T {
        <T as sealed::Sealed>::read(self)
    }

    /// write `value` to the config variable
    #[inline]
    pub fn write(&mut self, value: T) {
        <T as sealed::Sealed>::write(value, self)
    }
}

macro_rules! vars {
    ($($name:ident: $type:ty = $string:literal),*) => {
        /// config variable name
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[non_exhaustive]
        pub enum VarKind {
            $(
                #[doc = "`"]
                #[doc = $string]
                #[doc = "`"]
                // doc's alias is the same as it's name, cring
                //#[doc(alias = $string)]
                $name,
            )*
        }

        const VARS: phf::Map<&str, VarKind> = phf::phf_map! {
            $($string => VarKind::$name,)*
        };

        impl VarKind {
            /// Map a string to a variable kind.
            #[inline]
            pub fn from_str(var: &str) -> Option<Self> {
                VARS.get(var).copied()
            }

            /// Config variable name.
            #[inline]
            pub const fn name(&self) -> &'static str {
                match self {
                    $(VarKind::$name => concat!($string, "\0"),)*
                }
            }
        }

        /// Map of config variables.
        #[derive(Debug)]
        #[non_exhaustive]
        pub struct Vars {
            $(
                #[doc = "`"]
                #[doc = $string]
                #[doc = "`"]
                // doc's alias is the same as it's name, cring
                //#[doc(alias = $string)]
                pub $name: &'static mut Var<$type>,
            )*
        }

        impl Vars {
            /// load all config variables
            #[inline]
            pub fn from_loader<L>(mut loader: L) -> Result<Self, VarKind>
            where
                L: FnMut(VarKind) -> *mut (),
            {
                $(let $name = {
                    let var = loader(VarKind::$name).cast::<Var<$type>>();

                    if var.is_null() {
                        return Err(VarKind::$name);
                    }

                    unsafe { &mut *var }
                };)*

                Ok(Self { $($name,)* })
            }
        }
    };
}

vars! {
    alien_blood: bool = "violence_ablood",
    allow_developer: bool = "sv_max_allowed_developer",
    auto_help: bool = "cl_autohelp",

    cheats: bool = "sv_cheats",
    csm: bool = "cl_csm_enabled",
    csm_shadows: bool = "cl_csm_shadows",

    decals: bool = "r_drawdecals",

    engine_sleep: bool = "engine_no_focus_sleep",

    fast_render: bool = "cl_skipslowpath",
    ffa: bool = "mp_teammates_are_enemies",
    feet_shadows: bool = "cl_foot_contact_shadows",
    freeze_cam: bool = "cl_disablefreezecam",

    gravity: f32 = "sv_gravity",

    horizontal_speed: f32 = "cl_sidespeed",
    html_motd: bool = "cl_disablehtmlmotd",
    hud: bool = "cl_drawhud",
    vgui: bool = "r_drawvgui",
    human_blood: bool = "violence_hblood",

    infinite_ammo: f32 = "sv_infinite_ammo",
    interp: f32 = "cl_interp",
    interpolate: bool = "cl_interpolate",
    interp_ratio: f32 = "cl_interp_ratio",
    interp_ratio_min: f32 = "sv_client_min_interp_ratio",
    interp_ratio_max: f32 = "sv_client_max_interp_ratio",

    jiggle_bones: bool = "r_jiggle_bones",

    lag_comp: f32 = "cl_lagcompensation",

    developer: bool = "developer",

    max_commands: i32 = "sv_maxusrcmdprocessticks",

    other_models: i32 = "r_drawmodelstatsoverlay",

    panorama_blur: bool = "@panorama_disable_blur",
    physics_timescale: f32 = "cl_phys_timescale",
    prop_shadows: bool = "cl_csm_static_prop_shadows",

    rain: bool = "r_drawrain",
    recoil_scale: f32 = "weapon_recoil_scale",
    ragdoll_gravity: f32 = "cl_ragdoll_gravity",
    ropes: bool = "r_drawropes",
    rope_shadows: bool = "cl_csm_rope_shadows",

    shadows: bool = "r_shadows",
    show_grenade_path: bool = "cl_grenadepreview",
    show_help: bool = "cl_showhelp",
    show_impacts: bool = "sv_showimpacts",
    sprites: bool = "r_drawsprites",
    skybox3d: bool = "r_3dsky",
    sprite_shadows: bool = "cl_csm_sprite_shadows",

    timescale: f32 = "host_timescale",
    translucent_renderables: bool = "r_drawtranslucentrenderables",
    translucent_world: bool = "r_drawtranslucentworld",

    unlag_max: f32 = "sv_maxunlag",
    update_rate: f32 = "cl_updaterate",
    update_rate_max: f32 = "sv_maxupdaterate",
    underwater_overlay: bool = "r_drawunderwateroverlay",

    vertical_speed: f32 = "cl_forwardspeed",
    viewmodel_shadows: bool = "cl_csm_viewmodel_shadows",

    water_fog: bool = "fog_enable_water_fog",
    world_shadows: bool = "cl_csm_world_shadows"
}
