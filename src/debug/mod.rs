use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{
    bevy_egui::{EguiContext, EguiPlugin},
    bevy_inspector::ui_for_entity_with_children,
    egui,
};
use bevy_rapier2d::render::RapierDebugRenderPlugin;

use crate::level::CurrentLevel;

pub struct DebugPlugin {
    pub physics: bool,
    pub frame_time: bool,
    pub ui: bool,
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if self.ui {
            app.add_plugins(EguiPlugin)
                .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin)
                .add_systems(Update, debug_ui);
        }

        if self.physics {
            app.add_plugins(RapierDebugRenderPlugin::default());
        }
        if self.frame_time {
            app.add_plugins(FrameTimeDiagnosticsPlugin);
        }
    }
}

impl Default for DebugPlugin {
    fn default() -> Self {
        DebugPlugin {
            physics: false,
            frame_time: true,
            ui: true,
        }
    }
}

pub fn debug_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();
    let Some(level_entity) = world.resource::<CurrentLevel>().level_entity else {
        return;
    };

    egui::Window::new("UI").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Current Level");
            ui_for_entity_with_children(world, level_entity, ui);
        });
    });
}
