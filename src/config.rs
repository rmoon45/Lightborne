use bevy::prelude::*;
use serde::Deserialize;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        let config: Config = toml::from_str(
            &std::fs::read_to_string("Lightborne.toml")
                .expect("Failed to read Lightborne.toml. Is it in the right place?"),
        )
        .expect("Failed to parse Lightborne.toml. Is it formatted correctly?");
        app.insert_resource(config);
    }
}

#[derive(Deserialize, Resource)]
pub struct Config {
    pub level_config: LevelConfig,
}

#[derive(Deserialize)]
pub struct LevelConfig {
    pub level_index: usize,
    pub level_path: String,
}
