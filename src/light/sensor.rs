use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

use crate::{
    level::{
        activatable::ActiveActivatableGroup,
        interactable::{init_interactable, Interactable},
    },
    shared::GroupLabel,
};

use super::{HitByLight, LightSensor, LightSensorBundle};

impl From<&EntityInstance> for LightSensorBundle {
    fn from(entity_instance: &EntityInstance) -> Self {
        match entity_instance.identifier.as_ref() {
            "Button" => Self {
                collider: Collider::cuboid(4., 4.),
                sensor: Sensor,
                collision_groups: CollisionGroups::new(
                    GroupLabel::LIGHT_SENSOR,
                    GroupLabel::LIGHT_RAY | GroupLabel::WHITE_RAY,
                ),
                light_interaction: LightSensor {
                    activation_timer: Timer::new(Duration::from_millis(500), TimerMode::Once),
                    ..default()
                },
                interactable: init_interactable(entity_instance),
            },
            _ => unreachable!(),
        }
    }
}

pub fn update_light_sensors(
    mut commands: Commands,
    mut q_non_interactions: Query<&mut LightSensor, Without<HitByLight>>,
    mut q_interactions: Query<(&mut LightSensor, &Interactable), With<HitByLight>>,
    time: Res<Time>,
) {
    for mut sensor in q_non_interactions.iter_mut() {
        sensor.activation_timer.reset();
    }
    for (mut sensor, interactable) in q_interactions.iter_mut() {
        sensor.cumulative_exposure.tick(time.delta());
        sensor.activation_timer.tick(time.delta());

        if sensor.activation_timer.just_finished() {
            commands.spawn(ActiveActivatableGroup {
                id: interactable.id,
            });
        }
    }
}

pub fn clean_light_sensors(
    mut commands: Commands,
    q_hit_by_light: Query<Entity, With<HitByLight>>,
) {
    for entity in q_hit_by_light.iter() {
        commands.entity(entity).remove::<HitByLight>();
    }
}
