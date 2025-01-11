use bevy::{prelude::*, time::Stopwatch};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

use crate::{
    level::{
        activatable::GroupTriggeredEvent,
        interactable::{init_interactable, Interactable},
    },
    shared::GroupLabel,
};

#[derive(Default, Component)]
#[component(storage = "SparseSet")]
pub struct HitByLight;

#[derive(Component)]
pub struct LightSensor {
    /// Stores the cumulative time light has been hitting the sensor
    pub cumulative_exposure: Stopwatch,
    pub activation_timer: Timer,
    /// Whether or not the sensor was hit the previous frame
    pub was_hit: bool,
}

impl Default for LightSensor {
    fn default() -> Self {
        let mut timer = Timer::new(Duration::from_millis(300), TimerMode::Once);
        timer.pause();

        LightSensor {
            activation_timer: timer,
            cumulative_exposure: Stopwatch::default(),
            was_hit: false,
        }
    }
}

#[derive(Default, Bundle)]
pub struct LightSensorBundle {
    interactable: Interactable,
    collider: Collider,
    sensor: Sensor,
    collision_groups: CollisionGroups,
    light_interaction: LightSensor,
}

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
                light_interaction: LightSensor::default(),
                interactable: init_interactable(entity_instance),
            },
            _ => unreachable!(),
        }
    }
}

pub fn update_light_sensors(
    mut commands: Commands,
    mut q_non_interactions: Query<(&mut LightSensor, &Interactable), Without<HitByLight>>,
    mut q_interactions: Query<(Entity, &mut LightSensor, &Interactable), With<HitByLight>>,
    mut ev_group_triggered: EventWriter<GroupTriggeredEvent>,
    time: Res<Time>,
) {
    for (mut sensor, interactable) in q_non_interactions.iter_mut() {
        sensor.activation_timer.tick(time.delta());

        if sensor.was_hit {
            sensor.activation_timer.reset();
        }
        if sensor.activation_timer.just_finished() {
            ev_group_triggered.send(GroupTriggeredEvent {
                id: interactable.id,
            });
        }
        sensor.was_hit = false;
    }

    for (entity, mut sensor, interactable) in q_interactions.iter_mut() {
        if sensor.activation_timer.paused() {
            sensor.activation_timer.unpause();
        }

        sensor.cumulative_exposure.tick(time.delta());
        sensor.activation_timer.tick(time.delta());

        if !sensor.was_hit {
            sensor.activation_timer.reset();
        }
        if sensor.activation_timer.just_finished() {
            ev_group_triggered.send(GroupTriggeredEvent {
                id: interactable.id,
            });
        }
        sensor.was_hit = true;

        commands.entity(entity).remove::<HitByLight>();
    }
}
