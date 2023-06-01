use {
    bevy::{
        core_pipeline::{
            bloom::BloomSettings,
            tonemapping::{DebandDither, Tonemapping},
        },
        diagnostic::FrameTimeDiagnosticsPlugin,
        pbr::wireframe::WireframePlugin,
        prelude::*,
        render::{
            settings::{WgpuFeatures, WgpuSettings},
            RenderPlugin,
        },
        window::{PresentMode, WindowMode},
    },
    bevy_rapier3d::prelude::*,
    bevy_vfx_bag::{
        post_processing::{blur::Blur, chromatic_aberration::ChromaticAberration, wave::Wave},
        BevyVfxBagPlugin,
    },
    leafwing_input_manager::prelude::*,
};

/// Player input.
#[derive(Actionlike, Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PlayerInput {
    Attack,
    AttackSecondary,
    Crouch,
    Jump,
    Look,
    Move,
    Ping,
}

/// Client plugin.
#[derive(Default)]
pub struct EternalPlugin;

/// An entitys health.
#[derive(Clone, Copy, Debug, Deref, DerefMut, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Health(u32);

impl Plugin for EternalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(RenderPlugin {
                    wgpu_settings: WgpuSettings {
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..default()
                    },
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        decorations: false,
                        mode: WindowMode::BorderlessFullscreen,
                        present_mode: PresentMode::AutoNoVsync,
                        title: "Eternal".into(),
                        transparent: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugin(BevyVfxBagPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(InputManagerPlugin::<PlayerInput>::default())
        .add_plugin(RapierDebugRenderPlugin {
            always_on_top: true,
            enabled: false,
            ..default()
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(WireframePlugin)
        .add_startup_system(setup)
        .add_system(input);
    }
}

impl PlayerInput {
    /// Return the default `InputMap`.
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert(MouseButton::Left, Self::Attack);
        input_map.insert(MouseButton::Right, Self::AttackSecondary);
        input_map.insert(KeyCode::LShift, Self::Crouch);
        input_map.insert(KeyCode::Space, Self::Jump);
        input_map.insert(DualAxis::mouse_motion(), Self::Look);
        input_map.insert(VirtualDPad::wasd(), Self::Move);
        input_map.insert(MouseButton::Middle, Self::Ping);
        input_map
    }
}

#[bevy_main]
fn main() {
    App::new().add_plugin(EternalPlugin).run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        BloomSettings::NATURAL,
        Blur::default(),
        Camera3dBundle {
            dither: DebandDither::Enabled,
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        ChromaticAberration::default(),
        Wave {
            waves_x: 1.0,
            waves_y: 1.0,
            ..Wave::default()
        },
    ));

    commands.spawn(InputManagerBundle::<PlayerInput> {
        input_map: PlayerInput::default_input_map(),
        ..default()
    });
}

/// Main input system.
fn input(query: Query<&ActionState<PlayerInput>>) {
    let Ok(state) = query.get_single() else {
        return;
    };

    // Held inputs.
    let crouch = state.pressed(PlayerInput::Crouch);
    let jump = state.pressed(PlayerInput::Jump);

    // Pressed inputs.
    let attack = state.just_pressed(PlayerInput::Attack);
    let attack_secondary = state.just_pressed(PlayerInput::AttackSecondary);
    let ping = state.just_pressed(PlayerInput::Ping);

    // Rotation inputs.
    let rotation = state
        .axis_pair(PlayerInput::Look)
        .and_then(|axis| Some(axis.rotation()?.into_xy()))
        .unwrap_or_default();

    // Movement inputs.
    let movement = state
        .axis_pair(PlayerInput::Move)
        .and_then(|axis| Some(axis.direction()?.unit_vector()))
        .unwrap_or_default();

    let state = InputState {
        attack,
        attack_secondary,
        crouch,
        jump,
        ping,
        rotation,
        movement,
    };

    info!("{state:?}");
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct InputState {
    attack: bool,
    attack_secondary: bool,
    crouch: bool,
    jump: bool,
    ping: bool,
    rotation: Vec2,
    movement: Vec2,
}
