use bevy::{
    color::Color,
    ecs::{
        component::Component,
        system::{Commands, Query, Res},
    },
    input::{keyboard::KeyCode, ButtonInput},
    math::{Vec2, Vec3},
    sprite::Sprite,
    time::Time,
    transform::components::Transform,
    utils::hashbrown::HashMap,
};
use bevy_ggrs::{AddRollbackCommandExtension, LocalInputs, LocalPlayers, PlayerInputs};

use crate::session::{Config};

const PLAYER_ONE_COLOR: Color = Color::srgb(0., 0.47, 1.);
const PLAYER_TWO_COLOR: Color = Color::srgb(0., 0.4, 0.);
const PLAYER_SPEED: f32 = 7.;

#[derive(Component)]
pub struct Player {
    handle: usize,
}

pub fn spawn(mut commands: Commands) {
    // player 1
    commands
        .spawn((
            Player { handle: 0 },
            Transform::from_translation(Vec3::new(-2., 0., 0.)),
            Sprite {
                color: PLAYER_ONE_COLOR,
                custom_size: Some(Vec2::new(1., 1.)),
                ..Default::default()
            },
        ))
        .add_rollback();

    // player 2
    commands
        .spawn((
            Player { handle: 1 },
            Transform::from_translation(Vec3::new(2., 0., 0.)),
            Sprite {
                color: PLAYER_TWO_COLOR,
                custom_size: Some(Vec2::new(1., 1.)),
                ..Default::default()
            },
        ))
        .add_rollback();
}

// Input bits for player actions
// Each bit represents a different action in the input byte
const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;
const INPUT_FIRE: u8 = 1 << 4;

pub fn handle_input(
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

pub fn movement(
    mut players: Query<(&mut Transform, &Player)>,
    inputs: Res<PlayerInputs<Config>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut players {
        if let Some(inputs) = inputs.get(player.handle) {
            let (input, _) = inputs;

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
            if dir == Vec2::ZERO {
                continue;
            }

            let movement = dir * PLAYER_SPEED * time.delta_secs();
            transform.translation += movement.extend(0.);
        }
    }
}
