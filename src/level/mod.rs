use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::player::LdtkPlayerBundle;

use activatable::ActivatablePlugin;
use misc::on_door_changed;
use misc::ButtonBundle;
use misc::DoorBundle;
use walls::{spawn_wall_collision, WallBundle};

pub mod activatable;
pub mod interactable;

mod entity;
mod misc;
mod walls;

pub struct LevelManagementPlugin;

impl Plugin for LevelManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .add_plugins(ActivatablePlugin)
            .add_systems(Startup, setup_level)
            .insert_resource(LevelSelection::index(0))
            .register_ldtk_entity::<LdtkPlayerBundle>("Lyra")
            .register_ldtk_entity::<DoorBundle>("Door")
            .register_ldtk_entity::<ButtonBundle>("Button")
            .register_ldtk_int_cell::<WallBundle>(1)
            .add_systems(Update, spawn_wall_collision)
            .add_systems(Update, on_door_changed);
    }
}

fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("lightborne.ldtk").into(),
        ..Default::default()
    });
}
