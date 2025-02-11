use bevy::{
    ecs::system::{Commands, Res},
    input::{keyboard::KeyCode, ButtonInput},
    math::Vec2,
    utils::hashbrown::HashMap,
};
use bevy_ggrs::{LocalInputs, LocalPlayers};

use crate::session::Config;

// Input bits for player actions
// Each bit represents a different action in the input byte
const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;
const INPUT_FIRE: u8 = 1 << 4;

pub fn direction_from(input: &u8) -> Vec2 {
    let mut dir = Vec2::ZERO;
    if input & INPUT_UP != 0 {
        dir.y += 1.;
    }
    if input & INPUT_DOWN != 0 {
        dir.y -= 1.;
    }
    if input & INPUT_LEFT != 0 {
        dir.x -= 1.;
    }
    if input & INPUT_RIGHT != 0 {
        dir.x += 1.;
    }
    dir.normalize_or_zero()
}

pub fn handle(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    players: Res<LocalPlayers>,
) {
    let mut local_inputs = HashMap::new();

    for handle in &players.0 {
        let mut input = 0u8;

        if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            input |= INPUT_UP;
        }
        if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            input |= INPUT_DOWN;
        }
        if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            input |= INPUT_LEFT
        }
        if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            input |= INPUT_RIGHT;
        }
        if keys.any_pressed([KeyCode::Space, KeyCode::Enter]) {
            input |= INPUT_FIRE;
        }

        local_inputs.insert(*handle, input);
    }

    commands.insert_resource(LocalInputs::<Config>(local_inputs));
}

pub fn fire(input: &u8) -> bool {
    input & INPUT_FIRE != 0
}