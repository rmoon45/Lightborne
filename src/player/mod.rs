use bevy::{
    input::common_conditions::{input_just_pressed, input_pressed},
    prelude::*,
};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{input::update_cursor_world_coords, light::shoot::shoot_light};

use movement::{move_player, queue_jump, PlayerMovement};
use spawn::{add_player_sensors, init_player_bundle};

pub mod movement;
mod spawn;

pub struct PlayerManagementPlugin;
impl Plugin for PlayerManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_player_sensors) // ran when LDTK spawns the player
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

/// To signal our own code to finish the initialization of the player (adding sensors, etc)
#[derive(Component, Default)]
pub struct PlayerMarker;

#[derive(Default, Bundle)]
pub struct PlayerBundle {
    body: RigidBody,
    controller: KinematicCharacterController,
    controller_output: KinematicCharacterControllerOutput,
    collider: Collider,
    collision_groups: CollisionGroups,
    player_movement: PlayerMovement,
    friction: Friction,
    restitution: Restitution,
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct LdtkPlayerBundle {
    player_marker: PlayerMarker,
    #[with(init_player_bundle)]
    player: PlayerBundle,
    #[sprite("lyra.png")]
    sprite: Sprite,
    #[worldly]
    worldly: Worldly,
}
