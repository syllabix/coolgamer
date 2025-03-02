use bevy::prelude::*;
use character::CharacterPlugin;
use game_state::GameState;

mod character;
mod game_state;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Venture Time".to_string(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(CharacterPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2d);
}