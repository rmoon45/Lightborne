use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::shared::GroupLabel;

/// [`Bundle`] used to group together components commonly used together when initializing physics
/// for fixed [`LdtkEntity`]s.
#[derive(Default, Bundle)]
pub struct FixedEntityBundle {
    collider: Collider,
    rigid_body: RigidBody,
    collision_groups: CollisionGroups,
}

impl From<&EntityInstance> for FixedEntityBundle {
    /// This function will instantiate the proper values for a [`FixedEntityBundle`] depending on
    /// the [`LdtkEntity`]'s name in Ldtk. If you add a new entity in the Ldtk file that should be
    /// spawned with the [`FixedEntityBundle`], then you'll need to make changes here as well.
    fn from(entity_instance: &EntityInstance) -> Self {
        // NOTE: the size of the collider should match the visual of the entity in the level editor
        match entity_instance.identifier.as_ref() {
            "RedCrystal" | "GreenCrystal" => FixedEntityBundle {
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
