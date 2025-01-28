use bevy::{ecs::entity::EntityHashSet, prelude::*, time::Stopwatch};
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

/// [`Event`] used to notify other entities to trigger based on collision with light.
/// An included [`Entity`] is used to indicate the corresponding [`LightSensor`].
#[derive(Event)]
pub struct HitByLightEvent(pub Entity);

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

/// [`System`] that runs on [`Update`], querying each [`LightSensor`] and updating them
/// based on each [`HitByLightEvent`] generated in the [`System`]:
/// [`simulate_light_sources`](crate::light::segments::simulate_light_sources). This design
/// is still imperfect, as while it differs semantically from the previous implementation,
/// each [`Event`] is generated every frame. Preferably, refactor to include a "yap"-free
/// implementation across multiple systems to better utilize [`Event`].
pub fn update_light_sensors(
    mut commands: Commands,
    mut q_sensors: Query<(
        Entity,
        &mut LightSensor,
        &Interactable,
        Option<&InteractableSFX>,
    )>,
    mut ev_group_triggered: EventWriter<GroupTriggeredEvent>,
    mut ev_hit_by_light: EventReader<HitByLightEvent>,
    time: Res<Time>,
) {
    let mut hit_sensors: EntityHashSet = EntityHashSet::default();
    for ev in ev_hit_by_light.read() {
        hit_sensors.insert(ev.0);
    }

    for (entity, mut sensor, interactable, sfx) in q_sensors.iter_mut() {
        if hit_sensors.contains(&entity) {
            if sensor.activation_timer.paused() {
                sensor.activation_timer.unpause();
            }

            sensor.activation_timer.tick(time.delta());
            sensor.cumulative_exposure.tick(time.delta());

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
        } else {
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
    }
}
