use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crystal::CrystalPlugin;
use misc::ButtonBundle;

use crate::player::LdtkPlayerBundle;

use activatable::ActivatablePlugin;
use walls::{spawn_wall_collision, WallBundle};

pub mod activatable;
pub mod interactable;

mod crystal;
mod entity;
mod misc;
mod walls;

pub struct LevelManagementPlugin;

impl Plugin for LevelManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            // .insert_resource(LdtkSettings {
            //     level_background: LevelBackground::Nonexistent,
            //     ..default()
            // })
            .add_plugins(ActivatablePlugin)
            .add_plugins(CrystalPlugin)
            .add_systems(Startup, setup_level)
            .insert_resource(LevelSelection::index(0))
            .register_ldtk_entity::<LdtkPlayerBundle>("Lyra")
            .register_ldtk_entity::<ButtonBundle>("Button")
            .register_ldtk_int_cell::<WallBundle>(1)
            .add_systems(Update, spawn_wall_collision);
    }
}

fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("lightborne.ldtk").into(),
        ..Default::default()
    });
}
