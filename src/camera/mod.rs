use bevy::{prelude::*, render::camera::ScalingMode};

#[derive(Component, Default)]
pub struct MainCamera;

pub fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera2d)
        .insert(MainCamera)
        .insert(Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 360.,
            },
            ..OrthographicProjection::default_2d()
        }));
}
