use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{lighting::light::PointLighting, shared::GroupLabel};

use super::{
    light::PlayerLightInventory,
    movement::{PlayerMovement, PlayerState},
    PlayerBundle, PlayerMarker,
};

/// Attached to player hitbox
#[derive(Default, Component)]
pub struct PlayerHurtMarker;

/// Used by Ldtk to spawn the player correctly with all of the correct [`Component`]s.
pub fn init_player_bundle(_: &EntityInstance) -> PlayerBundle {
    PlayerBundle {
        body: RigidBody::KinematicPositionBased,
        controller: KinematicCharacterController {
            filter_groups: Some(CollisionGroups::new(
                GroupLabel::PLAYER_COLLIDER,
                GroupLabel::TERRAIN,
            )),
            offset: CharacterLength::Absolute(1.0),
            ..default()
        },
        controller_output: KinematicCharacterControllerOutput::default(),
        collider: Collider::cuboid(6.0, 9.0),
        collision_groups: CollisionGroups::new(GroupLabel::PLAYER_COLLIDER, GroupLabel::TERRAIN),
        player_movement: PlayerMovement::default(),
        player_state: PlayerState::Idle,
        friction: Friction {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Min,
        },
        restitution: Restitution {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Min,
        },
        light_inventory: PlayerLightInventory::default(),
        point_lighting: PointLighting {
            color: Vec3::new(0.8, 0.8, 0.8),
            radius: 40.0,
        },
    }
}

/// [`System`] that spawns the player's hurtbox [`Collider`] as a child entity.
pub fn add_player_sensors(mut commands: Commands, q_player: Query<Entity, Added<PlayerMarker>>) {
    let Ok(player) = q_player.get_single() else {
        return;
    };

    commands.entity(player).with_children(|parent| {
        parent
            .spawn(Collider::cuboid(5.0, 6.0))
            .insert(Sensor)
            .insert(RigidBody::Dynamic)
            .insert(GravityScale(0.0))
            .insert(PlayerHurtMarker)
            .insert(CollisionGroups::new(
                GroupLabel::PLAYER_SENSOR,
                GroupLabel::HURT_BOX | GroupLabel::TERRAIN,
            ))
            .insert(PointLight {
                intensity: 100_000.0,
                ..default()
            });
    });
}
