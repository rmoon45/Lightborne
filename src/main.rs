use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::ecs::schedule::{LogLevel, ScheduleBuildSettings};
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_rapier2d::prelude::*;

use camera::CameraPlugin;
use config::ConfigPlugin;
use input::{init_cursor_world_coords, update_cursor_world_coords};
use level::LevelManagementPlugin;
use light::LightManagementPlugin;
use player::PlayerManagementPlugin;

mod camera;
mod config;
mod input;
mod level;
mod light;
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
        .edit_schedule(Update, |schedule| {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Warn,
                ..default()
            });
        })
        .add_plugins(ConfigPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(8.0))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(PlayerManagementPlugin)
        .add_plugins(LevelManagementPlugin)
        .add_plugins(LightManagementPlugin)
        .add_plugins(CameraPlugin)
        .add_systems(Startup, init_cursor_world_coords)
        .add_systems(Update, update_cursor_world_coords)
        .run();
}
