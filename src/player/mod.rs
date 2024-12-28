use bevy::{input::common_conditions::input_pressed, prelude::*};
use bevy_ecs_ldtk::LdtkEntity;
use bevy_rapier2d::prelude::*;

use crate::input::update_cursor_world_coords;

use light::shoot_light;
use movement::move_player;
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
    Restitution
)]
pub struct Player {
    // Store my own velocity here, because KinematicCharacterController doesn't use Velocity (the
    // component) and it only supports setting translations
    velocity: Vec2,
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
