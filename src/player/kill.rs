use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    level::{misc::StartFlag, CurrentLevel},
    shared::{GameState, ResetLevel},
};

use super::{light::PlayerLightInventory, movement::PlayerMovement, PlayerMarker};

/// [`System`] that runs on [`GameState::Respawning`]. Will turn the state back into playing
/// immediately.
pub fn reset_player_position(
    mut q_player: Query<&mut Transform, With<PlayerMarker>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut ev_reset_level: EventReader<ResetLevel>,
    q_start_flag: Query<(&StartFlag, &EntityInstance)>,
    current_level: Res<CurrentLevel>,
) {
    // check that we recieved a ResetLevel event asking us to Respawn
    if !ev_reset_level.read().any(|x| *x == ResetLevel::Respawn) {
        return;
    }
    let Ok(mut transform) = q_player.get_single_mut() else {
        return;
    };

    next_game_state.set(GameState::Playing);

    for (flag, instance) in q_start_flag.iter() {
        if current_level.level_iid == flag.level_iid {
            transform.translation.x =
                instance.world_x.expect("Lightborne uses Free world layout") as f32;
            transform.translation.y =
                -instance.world_y.expect("Lightborne uses Free world layout") as f32;
            return;
        }
    }

    panic!("Couldn't find start flag to respawn at");
}

/// Resets the player inventory and movement information on a [`LevelSwitchEvent`]
pub fn reset_player_on_level_switch(
    mut q_player: Query<(&mut PlayerMovement, &mut PlayerLightInventory), With<PlayerMarker>>,
) {
    let Ok((mut movement, mut inventory)) = q_player.get_single_mut() else {
        return;
    };

    *movement = PlayerMovement::default();
    *inventory = PlayerLightInventory::default();
}
