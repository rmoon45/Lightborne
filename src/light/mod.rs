use bevy::{
    prelude::*,
    sprite::{AlphaMode2d, Material2dPlugin},
};

use enum_map::Enum;
use render::{insert_segment_meshes, LightMaterial};
use segments::{
    cleanup_light_sources, simulate_light_sources, tick_light_sources, LightSegmentCache,
};
use sensor::update_light_sensors;

mod render;
pub mod segments;
pub mod sensor;

const LIGHT_SPEED: f32 = 10.0;
const LIGHT_SEGMENT_THICKNESS: f32 = 3.0;

pub struct LightManagementPlugin;

impl Plugin for LightManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<LightMaterial>::default())
            .init_resource::<LightSegmentCache>()
            .add_systems(Update, simulate_light_sources)
            .add_systems(Update, insert_segment_meshes.after(simulate_light_sources))
            .add_systems(Update, update_light_sensors.after(simulate_light_sources))
            .add_systems(Update, cleanup_light_sources)
            .add_systems(FixedUpdate, tick_light_sources);
    }
}

#[derive(Enum, Clone, Copy, Default, PartialEq)]
pub enum LightColor {
    #[default]
    Green,
    Red,
    White,
}

impl From<LightColor> for Color {
    fn from(light_color: LightColor) -> Self {
        match light_color {
            LightColor::Red => Color::srgb(5.0, 0.0, 3.0),
            LightColor::Green => Color::srgb(3.0, 5.0, 0.0),
            LightColor::White => Color::srgb(2.0, 2.0, 2.0),
        }
    }
}

impl From<LightColor> for LightMaterial {
    fn from(light_color: LightColor) -> Self {
        let color = Color::from(light_color);
        LightMaterial {
            color: color.into(),
            alpha_mode: AlphaMode2d::Blend,
        }
    }
}

impl LightColor {
    pub fn num_bounces(&self) -> usize {
        match self {
            LightColor::Red => 2,
            _ => 1,
        }
    }
}

#[derive(Component)]
pub struct LightRaySource {
    pub start_pos: Vec2,
    pub start_dir: Vec2,
    pub time_traveled: f32,
    pub color: LightColor,
}
