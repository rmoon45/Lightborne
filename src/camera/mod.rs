use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    prelude::*,
    render::camera::ScalingMode,
};
use bevy_rapier2d::plugin::PhysicsSet;

use crate::{level::CurrentLevel, player::PlayerMarker};

/// The [`Plugin`] responsible for handling anything Camera related.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveCameraEvent>()
            .add_systems(Startup, setup_camera)
            .add_systems(PostUpdate, move_camera.after(PhysicsSet::Writeback)); // update after physics writeback to prevent jittering
    }
}

/// Marker [`Component`] used to query for the main (and currently only) camera in the world.
///
/// Your query might look like this:
/// ```rust
/// Query<&Transform, With<MainCamera>>
/// ```
#[derive(Component, Default)]
pub struct MainCamera;

/// [`Event`] struct used to request the camera move to a certain position, from another part of the
/// code.
///
/// Example usage:
/// ```rust
/// fn my_system(
///     mut ev_move_camera: EventWriter<MoveCameraEvent>,
/// ) {
///     ev_move_camera.send(MoveCameraEvent(Vec2::ZERO)); // will move the camera to 0, 0
/// }
/// ```
#[derive(Event)]
pub struct MoveCameraEvent(pub Vec2);

const CAMERA_WIDTH: f32 = 320.;
const CAMERA_HEIGHT: f32 = 180.;

/// [`Startup`] [`System`] that spawns the [`Camera2d`] in the world.
///
/// Notes:
/// - Spawns the camera with hardcoded position 160, -94
/// - Spawns the camera with [`OrthographicProjection`] with fixed scaling at 320x180
fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera2d)
        .insert(MainCamera)
        .insert(Camera {
            hdr: true,
            ..default()
        })
        .insert(Tonemapping::TonyMcMapface)
        .insert(Bloom::default())
        .insert(Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: CAMERA_WIDTH,
                height: CAMERA_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }))
        .insert(Transform::from_xyz(160., -94., 0.));
}

/// [`System`] that moves camera to player's position and constrains it to the [`CurrentLevel`]'s `world_box`.
pub fn move_camera(
    current_level: Res<CurrentLevel>,
    q_player: Query<&Transform, With<PlayerMarker>>,
    mut q_camera: Query<&mut Transform, (With<MainCamera>, Without<PlayerMarker>)>,
) {
    let Ok(mut camera_transform) = q_camera.get_single_mut() else {
        return;
    };
    let Ok(player_transform) = q_player.get_single() else {
        return;
    };
    let (x_min, x_max) = (
        current_level.world_box.min.x + CAMERA_WIDTH * 0.5,
        current_level.world_box.max.x - CAMERA_WIDTH * 0.5,
    );
    camera_transform.translation.x = player_transform.translation.x.max(x_min).min(x_max);

    let (y_min, y_max) = (
        current_level.world_box.min.y + CAMERA_HEIGHT * 0.5,
        current_level.world_box.max.y - CAMERA_HEIGHT * 0.5,
    );
    camera_transform.translation.y = player_transform.translation.y.max(y_min).min(y_max);
}
