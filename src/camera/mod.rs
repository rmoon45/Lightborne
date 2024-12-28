use bevy::{prelude::*, render::camera::ScalingMode};

use crate::player::Player;

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

pub fn center_camera_on_player(
    q_player: Query<&Transform, With<Player>>,
    mut q_camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let Ok(mut camera_transform) = q_camera.get_single_mut() else {
        return;
    };
    let Ok(player_transform) = q_player.get_single() else {
        return;
    };
    *camera_transform = *player_transform;
}
