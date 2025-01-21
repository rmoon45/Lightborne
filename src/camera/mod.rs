use std::time::Duration;

use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    prelude::*,
    render::camera::ScalingMode,
};
use bevy_rapier2d::plugin::PhysicsSet;

use crate::{level::CurrentLevel, player::PlayerMarker, shared::GameState};

/// The [`Plugin`] responsible for handling anything Camera related.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveCameraEvent>()
            .add_event::<CameraAnimationComplete>()
            .add_systems(Startup, setup_camera)
            .add_systems(FixedUpdate, move_camera.after(PhysicsSet::Writeback))
            .add_systems(
                Update,
                (handle_move_camera, on_camera_animation_complete).chain(),
            );
        // update after physics writeback to prevent jittering
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

const CAMERA_WIDTH: f32 = 320.;
const CAMERA_HEIGHT: f32 = 180.;
const CAMERA_ANIMATION_SECS: f32 = 0.4;

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

#[derive(Event)]
pub enum MoveCameraEvent {
    Animated(Vec2, Duration),
    Instant(Vec2),
}

#[derive(Event)]
pub struct CameraAnimationComplete;

pub struct Animation {
    progress: Timer,
    start: Vec3,
    end: Vec3,
}

impl Default for Animation {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(3.0, TimerMode::Once);
        timer.pause();

        Animation {
            progress: timer,
            start: Vec3::ZERO,
            end: Vec3::ZERO,
        }
    }
}

pub fn handle_move_camera(
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
    mut ev_move_camera: EventReader<MoveCameraEvent>,
    mut animation: Local<Animation>,
    mut ev_camera_animation: EventWriter<CameraAnimationComplete>,
    time: Res<Time>,
) {
    let Ok(mut camera_transform) = q_camera.get_single_mut() else {
        return;
    };

    for event in ev_move_camera.read() {
        match event {
            MoveCameraEvent::Animated(to, duration) => {
                animation.progress = Timer::new(*duration, TimerMode::Once);
                animation.start = camera_transform.translation;
                animation.end = to.extend(camera_transform.translation.z);
                dbg!("Starting camera anim");
            }
            MoveCameraEvent::Instant(to) => {
                camera_transform.translation = to.extend(camera_transform.translation.z);
            }
        }
    }
    if animation.progress.paused() {
        return;
    }
    animation.progress.tick(time.delta());
    if animation.progress.just_finished() {
        animation.progress.pause();
        ev_camera_animation.send(CameraAnimationComplete);
    }
    let percent = animation.progress.elapsed_secs() / animation.progress.duration().as_secs_f32();
    let f = EasingCurve::new(0.0, 1.0, EaseFunction::SineInOut);
    camera_transform.translation = animation
        .start
        .lerp(animation.end, f.sample_clamped(percent));
}

/// [`System`] that moves camera to player's position and constrains it to the [`CurrentLevel`]'s `world_box`.
pub fn move_camera(
    current_level: Res<CurrentLevel>,
    mut previous_level_iid: Local<String>,
    q_player: Query<&Transform, With<PlayerMarker>>,
    mut ev_move_camera: EventWriter<MoveCameraEvent>,
) {
    let Ok(player_transform) = q_player.get_single() else {
        return;
    };
    let (x_min, x_max) = (
        current_level.world_box.min.x + CAMERA_WIDTH * 0.5,
        current_level.world_box.max.x - CAMERA_WIDTH * 0.5,
    );
    let (y_min, y_max) = (
        current_level.world_box.min.y + CAMERA_HEIGHT * 0.5,
        current_level.world_box.max.y - CAMERA_HEIGHT * 0.5,
    );

    let new_pos = Vec2::new(
        player_transform.translation.x.max(x_min).min(x_max),
        player_transform.translation.y.max(y_min).min(y_max),
    );

    let event = if current_level.level_iid != *previous_level_iid && !previous_level_iid.is_empty()
    {
        MoveCameraEvent::Animated(new_pos, Duration::from_secs_f32(CAMERA_ANIMATION_SECS))
    } else {
        MoveCameraEvent::Instant(new_pos)
    };

    ev_move_camera.send(event);
    *previous_level_iid = current_level.level_iid.clone();
}

pub fn on_camera_animation_complete(
    mut ev_camera_animation: EventReader<CameraAnimationComplete>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if ev_camera_animation.is_empty() {
        return;
    }
    ev_camera_animation.clear();

    next_game_state.set(GameState::Playing);
}
