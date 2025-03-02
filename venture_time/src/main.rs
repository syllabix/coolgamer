use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_asset_loader::{
    asset_collection::AssetCollectionApp,
    loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt},
    standard_dynamic_asset::StandardDynamicAssetCollection,
};
use character::CharacterPlugin;
use game::{GameState, LaunchAssets, LoadingSequencePlugin};
use iyes_progress::ProgressPlugin;
use world::project_position;
use level::LevelPlugin;

mod character;
mod game;
mod world;
mod level;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Venture Time".to_string(),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            ProgressPlugin::<GameState>::new()
                .with_state_transition(GameState::AssetLoading, GameState::LaunchGame),
            FrameTimeDiagnosticsPlugin,
            LoadingSequencePlugin,
            CharacterPlugin,
            LevelPlugin,
        ))
        .init_state::<GameState>()
        .init_collection::<LaunchAssets>()
        .add_systems(Startup, setup)
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("config.ron")
                .load_collection::<character::Assets>()
                .load_collection::<level::Assets>(),
        )
        // .add_plugins(CharacterPlugin)
        .add_systems(Update, project_position)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2d);
}
