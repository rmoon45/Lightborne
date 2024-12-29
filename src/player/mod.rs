use std::time::Duration;

use bevy::{
    input::common_conditions::{input_just_pressed, input_pressed},
    prelude::*,
};
use bevy_ecs_ldtk::prelude::*;

use crate::{input::update_cursor_world_coords, light::shoot::shoot_light};

use movement::{move_player, queue_jump};
use spawn::process_player;

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

#[derive(Component)]
pub struct PlayerMovement {
    /// Holds information that is passed into the rapier character controller's translation
    velocity: Vec2,

    /// Started on the frame the player jumps. If this timer is still running, the player should
    /// not be able to cut their jump (to prevent super tiny jumps)
    prevent_jump_cut: Timer,

    /// Started on the frame the space bar is pressed. The player is will try to jump until the
    /// time expires, given that all other conditions are met.
    jump_queued: Timer,

    /// Started whenever the player is grounded. If the player attempts a jump even while not
    /// grounded, as long as this timer has not expired they are stil permitted to jump.
    coyote_time: Timer,
}

impl Default for PlayerMovement {
    fn default() -> Self {
        Self {
            jump_queued: Timer::new(Duration::from_millis(100), TimerMode::Once),
            coyote_time: Timer::new(Duration::from_millis(80), TimerMode::Once),
            prevent_jump_cut: Timer::new(Duration::from_millis(30), TimerMode::Once),
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
