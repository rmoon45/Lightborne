use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::shared::GroupLabel;

use super::{movement::PlayerMovement, PlayerMarker};

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
        .insert(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController {
            offset: CharacterLength::Absolute(1.0),
            ..default()
        })
        .insert(KinematicCharacterControllerOutput::default())
        .insert(Collider::cuboid(6.0, 9.0))
        .insert(CollisionGroups::new(
            GroupLabel::PLAYER_COLLIDER,
            GroupLabel::TERRAIN,
        ))
        .insert(PlayerMovement::default())
        .insert(Friction {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(Restitution {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Min,
        })
        .with_children(|parent| {
            parent
                .spawn(Collider::cuboid(5.0, 6.0))
                .insert(Sensor)
                .insert(CollisionGroups::new(
                    GroupLabel::PLAYER_SENSOR,
                    GroupLabel::HURT_SENSOR,
                ));
        });
}
