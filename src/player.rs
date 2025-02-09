use bevy::{
    color::Color, ecs::{
        component::Component, query::{With, Without}, system::{Commands, Query, Res}
    }, math::{Vec2, Vec3, Vec3Swizzles}, render::camera::Camera, sprite::Sprite, time::Time, transform::components::Transform
};
use bevy_ggrs::{AddRollbackCommandExtension, LocalPlayers, PlayerInputs};

use crate::{input::direction_from, map::MAP_SIZE, session::Config};

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
            Transform::from_translation(Vec3::new(-2., 0., 100.)),
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
            Transform::from_translation(Vec3::new(2., 0., 100.)),
            Sprite {
                color: PLAYER_TWO_COLOR,
                custom_size: Some(Vec2::new(1., 1.)),
                ..Default::default()
            },
        ))
        .add_rollback();
}

pub fn movement(
    mut players: Query<(&mut Transform, &Player)>,
    inputs: Res<PlayerInputs<Config>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut players {
        if let Some(inputs) = inputs.get(player.handle) {
            let (input, _) = inputs;

            let dir = direction_from(input);
            
            if dir == Vec2::ZERO {
                continue;
            }

            let movement = dir * PLAYER_SPEED * time.delta_secs();
            
            let old_pos = transform.translation.xy();
            let limit = Vec2::splat(MAP_SIZE as f32 / 2. - 0.5);
            let new_pos = (old_pos + movement).clamp(-limit, limit);

            transform.translation.x = new_pos.x;
            transform.translation.y = new_pos.y;
        }
    }
}

pub fn follow(
    local_players: Res<LocalPlayers>,
    players: Query<(&Player, &Transform)>,
    mut cameras: Query<&mut Transform, (With<Camera>, Without<Player>)>
) {
    for (player, player_transform) in &players {
        // only follow local player
        if !local_players.0.contains(&player.handle) {
            continue;
        }

        let pos = player_transform.translation;

        for mut transform in &mut cameras {
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    }
}