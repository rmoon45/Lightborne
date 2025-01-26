use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::shared::GameState;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Paused), spawn_pause)
            .add_systems(OnExit(GameState::Paused), despawn_pause)
            .add_systems(
                Update,
                toggle_pause.run_if(input_just_pressed(KeyCode::Escape)),
            );
    }
}

#[derive(Component)]
pub struct PauseMarker;

fn spawn_pause(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::linear_rgba(0., 0., 0., 0.0)),
            PauseMarker,
        ))
        .with_child((
            Text::new("Paused"),
            TextColor(Color::linear_rgb(1.0, 1.0, 1.0)),
        ));
}

fn despawn_pause(mut commands: Commands, query: Query<Entity, With<PauseMarker>>) {
    let Ok(entity) = query.get_single() else {
        return;
    };
    commands.entity(entity).despawn_recursive();
}

fn toggle_pause(state: Res<State<GameState>>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(match state.get() {
        GameState::Paused => GameState::Playing,
        GameState::Playing => GameState::Paused,
        x => x.clone(),
    })
}
