use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    prelude::*,
    render::camera::ScalingMode,
};

#[derive(Component, Default)]
pub struct MainCamera;

pub fn setup_camera(mut commands: Commands) {
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
        .insert(Transform::from_xyz(160., 94., 0.));
}
