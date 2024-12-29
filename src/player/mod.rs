use std::time::Duration;

use bevy::{
    input::common_conditions::{input_just_pressed, input_pressed},
    prelude::*,
    time::Stopwatch,
};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::input::update_cursor_world_coords;

use light::shoot_light;
use movement::{move_player, queue_jump};
use spawn::process_player;

mod light;
pub mod movement;
mod spawn;

pub struct PlayerManagementPlugin;
impl Plugin for PlayerManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, process_player) // ran when LDTK spawns the player
            .add_systems(FixedUpdate, move_player)
            .add_systems(
                Update,
                queue_jump
                    .run_if(input_just_pressed(KeyCode::Space))
                    .before(move_player),
            )
            .add_systems(
                Update,
                shoot_light
                    .run_if(input_pressed(MouseButton::Left))
                    .after(update_cursor_world_coords),
            );
    }
}

#[derive(Component, Default)]
#[require(
    Collider,
    KinematicCharacterController,
    KinematicCharacterControllerOutput,
    RigidBody,
    Friction,
    Restitution,
    PlayerMovement
)]
pub struct Player;

#[derive(Component)]
pub struct PlayerMovement {
    /// Holds information that is passed into the rapier character controller
    velocity: Vec2,
    /// Use `elapsed_secs()` to get the time since the last jump
    last_jumped: Stopwatch,
    /// Started on the frame the space bar is pressed,  the player is then only allowed to jump
    /// within the next window
    jump_queued: Timer,
}

impl Default for PlayerMovement {
    fn default() -> Self {
        Self {
            jump_queued: Timer::new(Duration::from_millis(100), TimerMode::Once),
            last_jumped: Stopwatch::new(),
            velocity: Vec2::ZERO,
        }
    }
}

/// To signal our own code to finish the initialization of the player
#[derive(Component, Default)]
pub struct PlayerMarker;

/// Will be spawned by LDTK. Player is technically a part of this bundle, but we want to spawn it
/// ourselves so it is not included here.
#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player_marker: PlayerMarker,
    #[sprite_sheet]
    sprite: Sprite,
}
