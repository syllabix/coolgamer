use bevy::{
    ecs::{
        component::Component,
        error::Result,
        query::{With, Without},
        system::{Commands, Query, Res},
    },
    image::TextureAtlas,
    input::keyboard::KeyCode,
    math::{Vec2, Vec3},
    prelude::{Camera, Transform, Window},
    reflect::Reflect,
    sprite::Sprite,
    window::PrimaryWindow,
};
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    Actionlike,
};

use crate::world::{Position, ZIndex};

use super::{
    attribute::{Direction, Jump, Movement},
    sprite::AnimationConfig,
    Assets,
};

const PLAYER_SPEED: f32 = 2.5;
const PLAYER_MAX_SPEED: f32 = 4.5;
const TILE_SIZE: f32 = 32.0;

// Ground level is three tiles up from the bottom edge
const GROUND_LEVEL_TILES: f32 = 3.0;

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    Jump,
    MoveLeft,
    MoveRight,
    Sprint,
}

/// Creates an input manager bundle with keyboard controls for character movement.
/// Maps the left and right arrow keys, A/D keys, and space bar to the corresponding actions.
/// Returns an `InputManagerBundle` configured with these mappings.
pub fn setup_player_controls() -> InputMap<Action> {
    InputMap::new([
        (Action::MoveLeft, KeyCode::ArrowLeft),
        (Action::MoveLeft, KeyCode::KeyA),
        (Action::MoveRight, KeyCode::ArrowRight),
        (Action::MoveRight, KeyCode::KeyD),
        (Action::Jump, KeyCode::Space),
        (Action::Sprint, KeyCode::ShiftLeft),
    ])
}

#[derive(Component, Default)]
#[require(
    Movement = Movement { speed: PLAYER_SPEED, ..Default::default() },
    Jump,
    Position = Position { coords: Vec2::ZERO, scale: Vec3::splat(4.0) },
    ZIndex = ZIndex(99)
)]
pub struct Player;

pub fn spawn(
    mut commands: Commands,
    assets: Res<Assets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) -> Result {
    let window = window_query.single()?;
    let start_x = -(window.width() / 2.0) + TILE_SIZE; // Left edge + one tile
    let ground_level = TILE_SIZE.mul_add(GROUND_LEVEL_TILES, -(window.height() / 2.0));

    let input = setup_player_controls();

    let sprite = Sprite::from_atlas_image(
        assets.venture_guy.clone(),
        TextureAtlas::from(assets.venture_guy_layout.clone()),
    );

    // TODO: move to config
    let animation = AnimationConfig::new(0, 6, 3, 20);

    commands.spawn((
        Player,
        input,
        sprite,
        animation,
        Position {
            coords: Vec2::new(start_x, ground_level),
            scale: Vec3::splat(4.0),
        },
        Movement {
            velocity: Vec2::ZERO,
            speed: PLAYER_SPEED,
            direction: Direction::Right,
        },
        Jump {
            is_jumping: false,
            jump_height: 0.0,
            jump_velocity: 0.0,
            gravity: 0.5,
            ground_level,
        },
        ZIndex(99),
    ));

    Ok(())
}

pub fn handle_input(
    mut player: Query<(&mut Movement, &mut Jump, &ActionState<Action>), With<Player>>,
) -> Result {
    let (mut movement, mut jump, action) = player.single_mut()?;
    let speed = if action.pressed(&Action::Sprint) {
        PLAYER_MAX_SPEED
    } else {
        PLAYER_SPEED
    };

    movement.speed = speed;

    if action.just_pressed(&Action::Jump) && !jump.is_jumping {
        jump.is_jumping = true;
        jump.jump_height = 0.0;
        jump.jump_velocity = 10.0;
    }

    if action.pressed(&Action::MoveLeft) {
        movement.velocity.x = -1.;
        movement.direction = Direction::Left;
        return Ok(());
    }

    if action.pressed(&Action::MoveRight) {
        movement.velocity.x = 1.;
        movement.direction = Direction::Right;
        return Ok(());
    }

    movement.velocity.x = 0.0;

    Ok(())
}

pub fn movement(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player: Query<(&Movement, &mut Position), With<Player>>,
) -> Result {
    let (movement, mut position) = player.single_mut()?;

    let window = window_query.single()?;

    // Calculate level boundaries (5x window width)
    let left_boundary = -(window.width() / 2.0);
    let right_boundary = window.width().mul_add(5.0, left_boundary);

    // Calculate new position
    let new_x = movement
        .velocity
        .x
        .mul_add(movement.speed, position.coords.x);

    // Clamp position within boundaries
    position.coords.x = new_x.clamp(left_boundary + TILE_SIZE, right_boundary - TILE_SIZE);

    // Update y position (for jumping)
    position.coords.y += movement.velocity.y * movement.speed;

    Ok(())
}

pub fn jump_physics(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player: Query<(&mut Jump, &mut Position), With<Player>>,
) -> Result {
    let (mut jump, mut position) = player.single_mut()?;
    // Update ground level based on window size
    let window = window_query.single()?;
    jump.ground_level = TILE_SIZE.mul_add(GROUND_LEVEL_TILES, -(window.height() / 2.0));

    if jump.is_jumping {
        // Apply jump velocity and gravity
        position.coords.y += jump.jump_velocity;
        jump.jump_height += jump.jump_velocity;
        jump.jump_velocity -= jump.gravity;

        // Check if we've returned to ground level
        if jump.jump_velocity < 0.0 && position.coords.y <= jump.ground_level {
            position.coords.y = jump.ground_level;
            jump.is_jumping = false;
            jump.jump_height = 0.0;
        }
    }

    Ok(())
}

pub fn camera_follow(
    player_query: Query<&Position, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) -> Result {
    let player_pos = player_query.single()?;
    let mut camera_transform = camera_query.single_mut()?;

    // Only start following when player moves past center
    let center_x = 0.0;
    if player_pos.coords.x > center_x {
        camera_transform.translation.x = player_pos.coords.x;
    }

    Ok(())
}
