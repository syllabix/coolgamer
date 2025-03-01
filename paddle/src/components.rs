//! Core game components and systems for the paddle game.
//!
//! This module contains the fundamental building blocks used to construct the game entities:
//! - Position and velocity components for movement
//! - Shape component for collision detection
//! - Ball, Paddle, and Gutter entities and their associated systems
//! - Spawn systems for creating game objects
//!
//! The components follow Bevy's ECS pattern and are designed to be composable.
//! Systems in this module handle core game mechanics like paddle movement and
//! ball physics.


use bevy::{log, prelude::*};
use config::{
    BALL_COLOR, BALL_SIZE, BALL_SPEED, GUTTER_HEIGHT, PADDLE_HEIGHT, PADDLE_ONE_COLOR,
    PADDLE_SPEED, PADDLE_TWO_COLOR, PADDLE_WIDTH,
};

use crate::input::{self};

mod config {
    use bevy::color::Color;

    pub const PADDLE_ONE_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
    pub const PADDLE_TWO_COLOR: Color = Color::srgb(0., 0., 1.);
    pub const PADDLE_SPEED: f32 = 4.;

    pub const BALL_SIZE: f32 = 5.0;
    pub const BALL_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);
    pub const BALL_SPEED: f32 = 5.;

    pub const PADDLE_WIDTH: f32 = 10.0;
    pub const PADDLE_HEIGHT: f32 = 50.0;

    pub const GUTTER_HEIGHT: f32 = 20.;
}

#[derive(Component, Default)]
#[require(Transform)]
pub struct Position {
    pub coords: Vec2,
}

impl Position {
    pub fn project(mut positionables: Query<(&mut Transform, &Position)>) {
        for (mut transform, position) in &mut positionables {
            transform.translation = position.coords.extend(0.)
        }
    }
}

#[derive(Component, Default)]
pub struct Velocity {
    pub direction: Vec2,
}

#[derive(Component, Default)]
pub struct Shape {
    pub size: Vec2,
}

#[derive(Component)]
#[require(
    Position,
    Velocity(|| Velocity { direction: Vec2::new(1., 1.)}),
    Shape(|| Shape { size: Vec2::new(BALL_SIZE, BALL_SIZE)})
)]
pub struct Ball;

impl Ball {
    pub fn spawn(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let shape = Circle::new(BALL_SIZE);
        let mesh = meshes.add(shape);
        let material = materials.add(BALL_COLOR);

        commands.spawn((Self, Mesh2d(mesh), MeshMaterial2d(material)));
    }

    pub fn movement(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
        if let Ok((mut position, velocity)) = ball.get_single_mut() {
            position.coords += velocity.direction * BALL_SPEED
        }
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Opponent;

#[derive(Component)]
#[require(
    Position,
    Velocity,
    Shape(|| Shape { size: Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)})
)]
pub struct Paddle;

pub fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    let window = if let Ok(window) = window.get_single() {
        window
    } else {
        log::error!("Failed to get window");
        return;
    };

    let width = window.resolution.width();
    let padding = 50.0;
    let right = width / 2.0 - padding;
    let left = -width / 2.0 + padding;

    let rect = Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT);
    let mesh = meshes.add(rect);
    let player_one_color = materials.add(PADDLE_ONE_COLOR);
    let player_two_color = materials.add(PADDLE_TWO_COLOR);

    commands.spawn((
        Player,
        Paddle,
        Position {
            coords: Vec2::new(right, 0.),
        },
        input::setup_controls(),
        Mesh2d(mesh.clone()),
        MeshMaterial2d(player_one_color),
    ));

    commands.spawn((
        Opponent,
        Paddle,
        Position {
            coords: Vec2::new(left, 0.),
        },
        Mesh2d(mesh.clone()),
        MeshMaterial2d(player_two_color),
    ));
}

pub fn move_player_paddle(
    mut paddle: Query<(&mut Position, &Velocity), With<Paddle>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let window_height = window.resolution.height();
        let max_y = (window_height / 2.) - GUTTER_HEIGHT - (PADDLE_HEIGHT / 2.);

        for (mut position, velocity) in &mut paddle {
            let new_position = position.coords + velocity.direction * PADDLE_SPEED;
            if new_position.y.abs() < max_y {
                position.coords = new_position;
            }
        }
    }
}

pub fn move_opponent_paddle(
    mut opponent: Query<(&Position, &mut Velocity), With<Opponent>>,
    ball: Query<&Position, With<Ball>>
) {
    if let Ok((position, mut velocity)) = opponent.get_single_mut() {
        if let Ok(ball_position) = ball.get_single() {
            let a_to_b = ball_position.coords - position.coords;
            velocity.direction.y = a_to_b.y.signum();
        }
    }
}

#[derive(Component)]
#[require(Position, Shape)]
struct Gutter;

pub fn spawn_gutters(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let window_width = window.resolution.width();
        let window_height = window.resolution.height();

        // We take half the window height because the center of our screen
        // is (0, 0). The padding would be half the height of the gutter as its
        // origin is also center rather than top left
        let top_gutter_y = window_height / 2. - GUTTER_HEIGHT / 2.;
        let bottom_gutter_y = -window_height / 2. + GUTTER_HEIGHT / 2.;

        let shape = Rectangle::from_size(Vec2::new(window_width, GUTTER_HEIGHT));
        let color = Color::srgb(0., 0., 0.);

        // We can share these meshes between our gutters by cloning them
        let mesh_handle = meshes.add(shape);
        let material_handle = materials.add(color);

        commands.spawn((
            Gutter,
            Shape { size: shape.size() },
            Position {
                coords: Vec2::new(0., top_gutter_y),
            },
            Mesh2d(mesh_handle.clone()),
            MeshMaterial2d(material_handle.clone()),
        ));

        commands.spawn((
            Gutter,
            Shape { size: shape.size() },
            Position {
                coords: Vec2::new(0., bottom_gutter_y),
            },
            Mesh2d(mesh_handle.clone()),
            MeshMaterial2d(material_handle.clone()),
        ));
    }
}
