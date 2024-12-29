use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Player, PlayerMarker};

pub fn process_player(
    mut commands: Commands,
    mut q_player: Query<(Entity, &mut Transform), Added<PlayerMarker>>,
) {
    let Ok((player, mut player_transform)) = q_player.get_single_mut() else {
        return;
    };

    // LDTK likes spawning the character funny...
    player_transform.scale = Vec3::ONE;

    commands
        .entity(player)
        .insert(Player::default())
        .insert(Collider::cuboid(8.0, 9.5))
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
