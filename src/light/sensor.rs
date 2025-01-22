use bevy::{prelude::*, time::Stopwatch};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

use crate::{
    level::{
        activatable::GroupTriggeredEvent,
        interactable::{init_interactable, Interactable, InteractableSFX},
    },
    shared::GroupLabel,
};

/// [`Component`] used to mark components that have been hit by light. The current design of the
/// system is very bad, an event like `HitByLightEvent(Entity)` should be used instead to signal
/// the [`LightSensor`]s to activate.
#[derive(Default, Component)]
#[component(storage = "SparseSet")]
pub struct HitByLight;

/// [`Component`] added to entities receptive to light. The
/// [`activation_timer`](LightSensor::activation_timer) should be initialized in the
/// `From<&EntityInstance>` implemenation for the [`LightSensorBundle`], if not default.
#[derive(Component)]
pub struct LightSensor {
    /// Stores the cumulative time light has been hitting the sensor
    pub cumulative_exposure: Stopwatch,
    /// The amount of time the light beam needs to be hitting the sensor for activation
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

/// [`Bundle`] that includes all the [`Component`]s needed for a [`LightSensor`] to function
/// properly.
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

/// [`System`] that resets the [`LightSensor`]s when a [`LevelSwitchEvent`] is received.
pub fn reset_light_sensors(mut q_sensors: Query<&mut LightSensor>) {
    for mut sensor in q_sensors.iter_mut() {
        sensor.activation_timer.reset();
        sensor.activation_timer.pause();
        sensor.was_hit = false;
        sensor.cumulative_exposure.reset();
    }
}

/// [`System`] that queries [`LightSensor`]s for [`HitByLight`] markers added when light hits
/// a light sensor, and immediately removes them. As mentioned in [`HitByLight`], this design
/// pattern isn't the best, and the [`Event`]-based one should be implemented instead.
pub fn update_light_sensors(
    mut commands: Commands,
    mut q_non_interactions: Query<(&mut LightSensor, &Interactable), Without<HitByLight>>,
    mut q_interactions: Query<
        (
            Entity,
            &mut LightSensor,
            &Interactable,
            Option<&InteractableSFX>,
        ),
        With<HitByLight>,
    >,
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

    for (entity, mut sensor, interactable, sfx) in q_interactions.iter_mut() {
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

            // FIXME: this feels rather hard coded, the interactable on_triggered effect sound
            // play should only have to be written once, in a system them handles all interactables
            // and not only light sensors
            if let Some(sfx) = sfx {
                if let Some(on_triggered) = &sfx.on_triggered {
                    commands.entity(entity).insert((
                        AudioPlayer::new(on_triggered.clone()),
                        PlaybackSettings::REMOVE,
                    ));
                }
            }
        }
        sensor.was_hit = true;

        commands.entity(entity).remove::<HitByLight>();
    }
}
