use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct LevelSetupPlugin;

impl Plugin for LevelSetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::index(3))
            // CHANGEME:                          ^
            // Change this if you want to start in a different level. Note that the "Lyra" entity
            // should be present in this level.
            .add_systems(Startup, setup_level);
    }
}

fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/lightborne.ldtk").into(),
        // CHANGEME:                           ^
        // Change this to the name of your own level file (likely
        // levels/<firstname>-<lastname>.ldtk)
        ..Default::default()
    });
}
