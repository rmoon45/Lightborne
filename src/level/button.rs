use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::light::LightMaterialBundle;

#[derive(Component, Default)]
pub struct Button;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct ButtonBundle {
    button: Button,
    #[from_int_grid_cell]
    light_material: LightMaterialBundle,
}
