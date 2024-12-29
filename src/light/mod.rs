use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::shared::GroupLabel;
use shoot::shoot_light;

pub mod shoot;

pub struct LightManagementPlugin;

impl Plugin for LightManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_light_interactions.after(shoot_light));
    }
}

#[derive(Default, Component)]
pub struct LightMaterial {
    /// Stores the cumulative time light has been hitting the sensor
    pub cumulative_exposure: Stopwatch,

    /// Timer that when finshed, indicates that light has been hitting this consecutively for the
    /// timer's duration
    pub activation_timer: Timer,

    /// Flag that is set to true when a light beam hits the entity. This is how we know if/when to
    /// tick timers or reset the activation timer
    pub currently_hit: bool,
}

#[derive(Default, Bundle, LdtkIntCell)]
pub struct LightMaterialBundle {
    collider: Collider,
    sensor: Sensor,
    collision_groups: CollisionGroups,
    light_interaction: LightMaterial,
}

impl From<IntGridCell> for LightMaterialBundle {
    fn from(int_grid_cell: IntGridCell) -> Self {
        // Temporary button
        if int_grid_cell.value == 5 {
            Self {
                collider: Collider::cuboid(4., 4.),
                sensor: Sensor,
                collision_groups: CollisionGroups::new(
                    GroupLabel::LIGHT_SENSOR,
                    GroupLabel::LIGHT_RAY,
                ),
                light_interaction: LightMaterial {
                    activation_timer: Timer::new(Duration::from_millis(500), TimerMode::Once),
                    ..default()
                },
            }
        } else {
            unreachable!();
        }
    }
}

pub fn update_light_interactions(mut q_interactions: Query<&mut LightMaterial>, time: Res<Time>) {
    for mut interaction in q_interactions.iter_mut() {
        if interaction.currently_hit {
            interaction.cumulative_exposure.tick(time.delta());
            interaction.activation_timer.tick(time.delta());
            interaction.currently_hit = false;

            if interaction.activation_timer.finished() {
                println!("Thing has been activated!")
            }
        } else {
            interaction.activation_timer.reset();
        }
    }
}
