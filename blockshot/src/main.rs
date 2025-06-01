use assets::ImageAssets;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_asset_loader::loading_state::{
    config::ConfigureLoadingState, LoadingState, LoadingStateAppExt,
};
use bevy_ggrs::{GgrsPlugin, GgrsSchedule, ReadInputs, RollbackApp};
use player::{Movement, Weapon};

mod assets;
mod input;
mod map;
mod player;
mod session;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
enum GameState {
    #[default]
    AssetLoading,
    Matchmaking,
    InGame,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Let's Play a Game".to_string(),
                    // fill the entire screen with the window
                    fit_canvas_to_parent: true,
                    // don't hijack keyboard shortcuts like Ctrl+R etc
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            GgrsPlugin::<session::Config>::default(),
        ))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .load_collection::<ImageAssets>()
                .continue_to_state(GameState::Matchmaking),
        )
        .rollback_component_with_clone::<Transform>()
        .rollback_component_with_copy::<Weapon>()
        .rollback_component_with_copy::<Movement>()
        .insert_resource(ClearColor(Color::srgb(0.53, 0.53, 0.53)))
        .add_systems(OnEnter(GameState::Matchmaking), (setup, session::connect))
        .add_systems(OnEnter(GameState::InGame), player::spawn)
        .add_systems(
            Update,
            (
                session::wait_for_players.run_if(in_state(GameState::Matchmaking)),
                player::follow.run_if(in_state(GameState::InGame)),
            ),
        )
        .add_systems(ReadInputs, input::handle)
        .add_systems(
            GgrsSchedule,
            (
                player::movement,
                player::reload,
                player::attack,
                player::move_bullet,
                player::kill,
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 10.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    map::setup(commands);
}
