use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::player::PlayerBundle;

use button::ButtonBundle;
use walls::{spawn_wall_collision, WallBundle};

mod button;
mod walls;

pub struct LevelManagementPlugin;

impl Plugin for LevelManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            // .insert_resource(LdtkSettings {
            //     level_background: LevelBackground::Nonexistent,
            //     ..default()
            // })
            .add_systems(Startup, setup_level)
            .insert_resource(LevelSelection::index(0))
            .register_ldtk_entity::<PlayerBundle>("Lyra")
            .register_ldtk_int_cell::<WallBundle>(1)
            .register_ldtk_int_cell::<ButtonBundle>(5)
            .add_systems(Update, spawn_wall_collision);
    }
}

fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("lightborne.ldtk").into(),
        ..Default::default()
    });
}
