use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    prelude::*,
    render::camera::ScalingMode,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveCameraEvent>()
            .add_systems(Startup, setup_camera)
            .add_systems(Update, move_camera);
    }
}

#[derive(Component, Default)]
pub struct MainCamera;

#[derive(Event)]
pub struct MoveCameraEvent(pub Vec2);

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
