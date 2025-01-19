use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    prelude::*,
    render::camera::ScalingMode,
};

/// The [`Plugin`] responsible for handling anything Camera related.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveCameraEvent>()
            .add_systems(Startup, setup_camera)
            .add_systems(Update, move_camera);
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
                width: 320.,
                height: 180.,
            },
            ..OrthographicProjection::default_2d()
        }))
        .insert(Transform::from_xyz(160., -94., 0.));
}

/// [`System`] that responds to [`MoveCameraEvent`] events. Will assign the value of the latest event
/// read this frame to the [`MainCamera`]'s [`Transform`].
pub fn move_camera(
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
    mut ev_move_camera: EventReader<MoveCameraEvent>,
) {
    let Ok(mut transform) = q_camera.get_single_mut() else {
        return;
    };
    if ev_move_camera.is_empty() {
        return;
    }
    let mut next_pos = transform.translation.truncate();
    for event in ev_move_camera.read() {
        next_pos = event.0;
    }
    transform.translation = next_pos.extend(0.0);
}
