use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Player, PlayerMarker};

pub fn process_player(mut commands: Commands, q_player: Query<Entity, Added<PlayerMarker>>) {
    let Ok(player) = q_player.get_single() else {
        return;
    };
    commands
        .entity(player)
        .insert(Player::default())
        .insert(Collider::cuboid(8.0, 8.0))
        .insert(RigidBody::KinematicPositionBased)
        .insert(Friction {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(Restitution {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Min,
        });
}
