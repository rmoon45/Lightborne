use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    player::{movement::PlayerMovement, PlayerMarker},
    shared::GroupLabel,
};

#[derive(Default, Component)]
pub struct SemiSolid;

/// Bundle for Semi-Solid Platforms
#[derive(Default, Bundle, LdtkIntCell)]
pub struct SemiSolidPlatformBundle {
    #[from_int_grid_cell]
    fixed_entity_bundle: FixedEntityBundle,
    semi_solid: SemiSolid,
}

pub fn adjust_semisolid_colliders(mut q_semisolid: Query<&mut Transform, Added<SemiSolid>>) {
    for mut transform in q_semisolid.iter_mut() {
        // if transform.translation.y % 8. == 0. {
        transform.translation.y += 3.;
        // }
    }
}

/// Sets the state of SemiSolids based on Player's y coord
pub fn set_semisolid(
    q_player: Query<(&PlayerMovement, &GlobalTransform), With<PlayerMarker>>,
    mut q_semisolid: Query<(&GlobalTransform, &mut CollisionGroups), With<SemiSolid>>,
) {
    let Ok((movement, player)) = q_player.get_single() else {
        return;
    };
    const PLAYER_HALF_HEIGHT: f32 = 9.0;
    let cutoff_height = if movement.crouching {
        PLAYER_HALF_HEIGHT / 2.0
    } else {
        PLAYER_HALF_HEIGHT
    };

    for (transform, mut collisions) in q_semisolid.iter_mut() {
        if player.compute_transform().translation.y - transform.compute_transform().translation.y
            > cutoff_height
        {
            *collisions = CollisionGroups::new(GroupLabel::TERRAIN, GroupLabel::ALL);
        } else {
            *collisions = CollisionGroups::new(
                GroupLabel::TERRAIN,
                GroupLabel::ALL & !GroupLabel::PLAYER_COLLIDER,
            );
        }
    }
}

/// Component for things that hurt
#[derive(Default, Component)]
pub struct HurtMarker;

/// Component for spikes
#[derive(Default, Component)]
pub struct Spike {
    // name: String,
    num_deaths: u32,
}

/// method to increase num_deaths of spike
impl Spike {
    pub fn add_death(&mut self) {
        self.num_deaths += 1;
    }
}

/// IntGrid implementation of Spike
impl From<IntGridCell> for Spike {
    fn from(cell_instance: IntGridCell) -> Self {
        match cell_instance.value {
            2 => Spike {
                // name: "baseSpike".to_string(),
                num_deaths: 0,
            },
            _ => unreachable!(),
        }
    }
}

/// Bundle for spikes
#[derive(Default, Bundle, LdtkIntCell)]
pub struct SpikeBundle {
    #[from_int_grid_cell]
    fixed_entity_bundle: FixedEntityBundle,
    hurt_marker: HurtMarker,
    spike: Spike,
}

/// [`Bundle`] used to group together components commonly used together when initializing physics
/// for fixed [`LdtkEntity`]s.
#[derive(Default, Bundle)]
pub struct FixedEntityBundle {
    collider: Collider,
    rigid_body: RigidBody,
    collision_groups: CollisionGroups,
}

/// IntGrid implementation of FixedEntityBundle
impl From<IntGridCell> for FixedEntityBundle {
    fn from(cell_instance: IntGridCell) -> Self {
        match cell_instance.value {
            2 => FixedEntityBundle {
                collider: Collider::triangle(
                    Vec2::new(-4., -4.),
                    Vec2::new(4., -4.),
                    Vec2::new(0., 4.),
                ),
                rigid_body: RigidBody::Fixed,
                collision_groups: CollisionGroups::new(
                    GroupLabel::TERRAIN,
                    GroupLabel::ALL & !GroupLabel::PLAYER_COLLIDER,
                ),
            },
            15 => FixedEntityBundle {
                collider: Collider::cuboid(4., 1.),
                rigid_body: RigidBody::Fixed,
                collision_groups: CollisionGroups::new(GroupLabel::TERRAIN, GroupLabel::ALL),
            },
            _ => unreachable!(),
        }
    }
}
