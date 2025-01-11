use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::light::sensor::LightSensorBundle;

#[derive(Default, Component)]
pub struct ButtonMarker;

#[derive(Default, Bundle, LdtkEntity)]
pub struct ButtonBundle {
    marker: ButtonMarker,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[from_entity_instance]
    light_sensor: LightSensorBundle,
}
