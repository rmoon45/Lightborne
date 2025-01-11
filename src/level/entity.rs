use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::shared::GroupLabel;

#[derive(Default, Bundle)]
pub struct FixedEntityBundle {
    collider: Collider,
    rigid_body: RigidBody,
    collision_groups: CollisionGroups,
}

impl From<&EntityInstance> for FixedEntityBundle {
    fn from(entity_instance: &EntityInstance) -> Self {
        // NOTE: the size of the collider should match the visual of the entity in the level editor
        match entity_instance.identifier.as_ref() {
            "Door" => FixedEntityBundle {
                collider: Collider::cuboid(4., 4.),
                rigid_body: RigidBody::Fixed,
                collision_groups: CollisionGroups::new(
                    GroupLabel::TERRAIN,
                    GroupLabel::LIGHT_RAY | GroupLabel::PLAYER_COLLIDER | GroupLabel::WHITE_RAY,
                ),
            },
            _ => unreachable!(),
        }
    }
}
