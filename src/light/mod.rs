use bevy::{prelude::*, time::Stopwatch};
use bevy_rapier2d::prelude::*;

use sensor::update_light_sensors;
use shoot::shoot_light;

use crate::level::interactable::Interactable;

pub mod sensor;
pub mod shoot;

pub struct LightManagementPlugin;

impl Plugin for LightManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_light_sensors.after(shoot_light));
    }
}

#[derive(Default, Component)]
#[component(storage = "SparseSet")]
pub struct HitByLight;

#[derive(Default, Component)]
pub struct LightSensor {
    /// Stores the cumulative time light has been hitting the sensor
    pub cumulative_exposure: Stopwatch,

    /// Timer that when finshed, indicates that light has been hitting this consecutively for the
    /// timer's duration
    pub activation_timer: Timer,
}

#[derive(Default, Bundle)]
pub struct LightSensorBundle {
    interactable: Interactable,
    collider: Collider,
    sensor: Sensor,
    collision_groups: CollisionGroups,
    light_interaction: LightSensor,
}
