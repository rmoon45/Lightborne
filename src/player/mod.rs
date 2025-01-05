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
