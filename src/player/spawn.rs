use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::shared::GroupLabel;

use super::{light::PlayerLightInventory, movement::PlayerMovement, PlayerBundle, PlayerMarker};

pub fn init_player_bundle(_: &EntityInstance) -> PlayerBundle {
    PlayerBundle {
        body: RigidBody::KinematicPositionBased,
        controller: KinematicCharacterController {
            offset: CharacterLength::Absolute(1.0),
            ..default()
        },
        controller_output: KinematicCharacterControllerOutput::default(),
        collider: Collider::cuboid(6.0, 9.0),
        collision_groups: CollisionGroups::new(GroupLabel::PLAYER_COLLIDER, GroupLabel::TERRAIN),
        player_movement: PlayerMovement::default(),
        friction: Friction {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Min,
        },
        restitution: Restitution {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Min,
        },
        light_inventory: PlayerLightInventory::default(),
    }
}

pub fn add_player_sensors(mut commands: Commands, q_player: Query<Entity, Added<PlayerMarker>>) {
    let Ok(player) = q_player.get_single() else {
        return;
    };

    commands.entity(player).with_children(|parent| {
        parent
            .spawn(Collider::cuboid(5.0, 6.0))
            .insert(Sensor)
            .insert(CollisionGroups::new(
                GroupLabel::PLAYER_SENSOR,
                GroupLabel::HURT_BOX,
            ))
            .insert(PointLight {
                intensity: 100_000.0,
                ..default()
            });
    });
}
