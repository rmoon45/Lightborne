use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_rapier2d::prelude::*;

use camera::CameraPlugin;
use config::ConfigPlugin;
use debug::DebugPlugin;
use input::{init_cursor_world_coords, update_cursor_world_coords};
use level::LevelManagementPlugin;
use light::LightManagementPlugin;
use pause::PausePlugin;
use player::PlayerManagementPlugin;
use shared::{GameState, ResetLevel};

mod camera;
mod config;
mod debug;
mod input;
mod level;
mod light;
mod pause;
mod player;
mod shared;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Lightborne".into(),
                        name: Some("lightborne".into()),
                        present_mode: PresentMode::AutoNoVsync,
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(ConfigPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(8.0).in_fixed_schedule())
        .add_plugins(PlayerManagementPlugin)
            .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(LevelManagementPlugin)
        .add_plugins(LightManagementPlugin)
        .add_plugins(PausePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DebugPlugin::default())
        .insert_state(GameState::Playing)
        .add_event::<ResetLevel>()
        .add_systems(Startup, init_cursor_world_coords)
        .add_systems(Update, update_cursor_world_coords)
        .run();
}
