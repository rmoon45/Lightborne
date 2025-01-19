use bevy::prelude::*;
use serde::Deserialize;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        let config_path = if let Ok(true) = std::fs::exists("Lightborne.toml") {
            "Lightborne.toml"
        } else {
            "Lightborne_example.toml"
        };
        let config: Config = toml::from_str(
            &std::fs::read_to_string(config_path)
                .expect(&format!("Failed to find {config_path}. Is it in the right place?")),
        )
        .expect(&format!("Failed to parse {config_path}. Is it formatted correctly?"));
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
