use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ggrs::{GgrsApp, GgrsPlugin, GgrsSchedule, ReadInputs};

mod player;
mod session;
mod input;
mod map;

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
        .rollback_component_with_clone::<Transform>()
        .insert_resource(ClearColor(Color::srgb(0.53, 0.53, 0.53)))
        .add_systems(Startup, (setup, player::spawn, session::connect))
        .add_systems(Update, (session::wait_for_players, player::follow))
        .add_systems(ReadInputs, input::handle)
        .add_systems(GgrsSchedule, player::movement)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 10.0,
            },
            ..OrthographicProjection::default_2d()
        }
    ));

    map::setup(commands);
}
