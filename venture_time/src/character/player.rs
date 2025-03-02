use bevy::{
    ecs::{
        component::Component,
        query::With,
        system::{Commands, Query, Res},
    },
    input::keyboard::KeyCode,
    math::{Vec2, Vec3},
    reflect::Reflect,
    sprite::{Sprite, TextureAtlas},
};
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    Actionlike, InputManagerBundle,
};

use crate::world::Position;

use super::{
    attribute::{Direction, Health, Jump, Movement},
    sprite::{gabe, AnimationConfig},
};

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
/// Returns an InputManagerBundle configured with these mappings.
pub fn setup_player_controls() -> InputManagerBundle<Action> {
    InputManagerBundle::with_map(InputMap::new([
        (Action::MoveLeft, KeyCode::ArrowLeft),
        (Action::MoveLeft, KeyCode::KeyA),
        (Action::MoveRight, KeyCode::ArrowRight),
        (Action::MoveRight, KeyCode::KeyD),
        (Action::Jump, KeyCode::Space),
        (Action::Sprint, KeyCode::ShiftLeft),
    ]))
}

const PLAYER_SPEED: f32 = 2.5;
const PLAYER_MAX_SPEED: f32 = 4.5;

#[derive(Component, Default)]
#[require(
    Health,
    Movement(|| Movement { speed: PLAYER_SPEED, ..Default::default() }),
    Jump,
    Position(|| Position { coords: Vec2::ZERO, scale: Vec3::splat(4.0) }),
)]
pub struct Player;

pub fn spawn(mut commands: Commands, gabe_sprite: Res<gabe::SpriteConfig>) {
    let sprite = Sprite {
        image: gabe_sprite.image.clone(),
        texture_atlas: Some(TextureAtlas {
            layout: gabe_sprite.texture_atlas_layout.clone(),
            index: gabe_sprite.first_index,
        }),
        ..Default::default()
    };

    let animation = AnimationConfig::new(
        gabe_sprite.first_index,
        gabe_sprite.last_index,
        gabe_sprite.fps,
    );

    let input = setup_player_controls();

    commands.spawn((Player, input, sprite, animation));
}

pub fn handle_input(mut player: Query<(&mut Movement, &mut Jump, &ActionState<Action>), With<Player>>) {
    if let Ok((mut movement, mut jump, action)) = player.get_single_mut() {
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
            return;
        }

        if action.pressed(&Action::MoveRight) {
            movement.velocity.x = 1.;
            movement.direction = Direction::Right;
            return;
        }

        movement.velocity.x = 0.0;
    }
}

pub fn movement(mut player: Query<(&Movement, &mut Position), With<Player>>) {
    if let Ok((movement, mut position)) = player.get_single_mut() {
        // Update position based on velocity
        position.coords += movement.velocity * movement.speed;
    }
}

pub fn jump_physics(mut player: Query<(&mut Jump, &mut Position), With<Player>>) {
    if let Ok((mut jump, mut position)) = player.get_single_mut() {
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
    }
}
