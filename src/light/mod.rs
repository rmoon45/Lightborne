use bevy::{prelude::*, sprite::Material2dPlugin, time::Stopwatch};
use bevy_rapier2d::prelude::*;

use render::{clean_segments, insert_segment_meshes, LightMaterial, LightRenderData};
use sensor::{clean_light_sensors, update_light_sensors};
use shoot::shoot_light;

use crate::level::interactable::Interactable;

mod render;
pub mod sensor;
pub mod shoot;

const MAX_LIGHT_SEGMENTS: usize = 3;
const SEGMENT_THICKNESS: f32 = 2.0;

pub struct LightManagementPlugin;

impl Plugin for LightManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<LightMaterial>::default())
            .init_resource::<LightRenderData>()
            .add_systems(Update, update_light_sensors.after(shoot_light))
            .add_systems(Update, insert_segment_meshes.after(shoot_light))
            .add_systems(PostUpdate, clean_light_sensors)
            .add_systems(PreUpdate, clean_segments);
    }
}

#[derive(Default, Component)]
#[component(storage = "SparseSet")]
pub struct LightSegment {
    start: Vec2,
    end: Vec2,
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
