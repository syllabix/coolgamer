use bevy::prelude::*;
use components::{move_opponent_paddle, move_player_paddle, spawn_gutters, spawn_paddles, Ball, Position};
use game::{detect_scoring, reset_ball, update_score};
use leafwing_input_manager::plugin::InputManagerPlugin;

mod components;
mod input;
mod physics;
mod game;
mod scoreboard;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Paddle"),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: true,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(InputManagerPlugin::<input::Action>::default())
        .init_resource::<game::Score>()
        .add_event::<game::PointScored>()
        .add_systems(
            Startup,
            (
                setup,
                Ball::spawn.after(setup),
                spawn_paddles.after(setup),
                spawn_gutters.after(setup),
                scoreboard::spawn.after(setup),
            ),
        )
        .add_systems(
            Update,
            (
                // ball related systems
                Ball::movement,
                Position::project.after(Ball::movement),
                physics::handle_collisions.after(Ball::movement),
                
                // input related systems
                input::handle,

                // game state management
                detect_scoring,
                reset_ball.after(detect_scoring),
                update_score.after(detect_scoring),
                scoreboard::update,

                // paddle related systems
                move_player_paddle.after(input::handle),
                move_opponent_paddle
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2d);
}
