use bevy::{
    color::Color,
    ecs::{
        component::Component, entity::Entity, query::{With, Without}, system::{Commands, Query, Res}
    },
    math::{Quat, Vec2, Vec3, Vec3Swizzles},
    render::camera::Camera,
    sprite::Sprite,
    time::Time,
    transform::components::Transform,
};
use bevy_ggrs::{AddRollbackCommandExtension, LocalPlayers, PlayerInputs};

use crate::{
    assets::ImageAssets,
    input::{direction_from, fire},
    map::MAP_SIZE,
    session::Config,
};

const PLAYER_ONE_COLOR: Color = Color::srgb(0., 0.47, 1.);
const PLAYER_TWO_COLOR: Color = Color::srgb(0., 0.4, 0.);
const PLAYER_SPEED: f32 = 7.;

#[derive(Component)]
pub struct Player {
    handle: usize,
}

#[derive(Component, Clone, Copy)]
pub struct Movement {
    pub direction: Vec2,
}

pub fn spawn(mut commands: Commands) {
    // player 1
    commands
        .spawn((
            Player { handle: 0 },
            Weapon { ready: true },
            Movement {
                direction: -Vec2::X,
            },
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
            Weapon { ready: true },
            Movement { direction: Vec2::X },
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
    mut players: Query<(&mut Transform, &Player, &mut Movement)>,
    inputs: Res<PlayerInputs<Config>>,
    time: Res<Time>,
) {
    for (mut transform, player, mut movement) in &mut players {
        if let Some(inputs) = inputs.get(player.handle) {
            let (input, _) = inputs;

            let dir = direction_from(input);

            if dir == Vec2::ZERO {
                continue;
            }

            movement.direction = dir;

            let movement = dir * PLAYER_SPEED * time.delta_secs();

            let old_pos = transform.translation.xy();
            let limit = Vec2::splat(MAP_SIZE as f32 / 2. - 0.5);
            let new_pos = (old_pos + movement).clamp(-limit, limit);

            transform.translation.x = new_pos.x;
            transform.translation.y = new_pos.y;
        }
    }
}

#[derive(Component)]
pub struct Bullet;

#[derive(Component, Clone, Copy)]
pub struct Weapon {
    pub ready: bool,
}

pub fn attack(
    mut commands: Commands,
    inputs: Res<PlayerInputs<Config>>,
    images: Res<ImageAssets>,
    mut players: Query<(&Transform, &Player, &mut Weapon, &Movement)>,
) {
    for (transform, player, mut weapon, movement) in &mut players {
        let player_position = transform.translation.xy();
        let pos = player_position + movement.direction * PLAYER_RADIUS + BULLET_RADIUS;
        let (input, _) = inputs[player.handle];
        if fire(&input) && weapon.ready {
            commands
                .spawn((
                    Bullet,
                    Transform::from_translation(pos.extend(200.))
                        .with_rotation(Quat::from_rotation_arc_2d(Vec2::X, movement.direction)),
                    *movement,
                    Sprite {
                        image: images.bullet.clone(),
                        custom_size: Some(Vec2::new(0.3, 0.1)),
                        ..Default::default()
                    },
                ))
                .add_rollback();
            weapon.ready = false;
        }
    }
}

pub fn move_bullet(mut bullets: Query<(&mut Transform, &Movement), With<Bullet>>, time: Res<Time>) {
    for (mut transform, movement) in &mut bullets {
        let speed = 20.0;
        let delta = movement.direction * speed * time.delta_secs();
        transform.translation += delta.extend(0.);
    }
}

const PLAYER_RADIUS: f32 = 0.5;
const BULLET_RADIUS: f32 = 0.025;

pub fn kill(
    mut commands: Commands,
    players: Query<(Entity, &Transform), (With<Player>, Without<Bullet>)>,
    bullets: Query<&Transform, With<Bullet>>,
) {
    for (player, player_transform) in &players {
        for bullet_transform in &bullets {
            let distance = Vec2::distance(
                player_transform.translation.xy(),
                bullet_transform.translation.xy(),
            );
            if distance < PLAYER_RADIUS + BULLET_RADIUS {
                commands.entity(player).despawn();
            }
        }
    }
}


pub fn reload(inputs: Res<PlayerInputs<Config>>, mut players: Query<(&mut Weapon, &Player)>) {
    for (mut can_fire, player) in &mut players {
        if let Some(handle) = inputs.get(player.handle) {
            let (input, _) = handle;
            if !fire(input) {
                can_fire.ready = true;
            }
        }
    }
}

pub fn follow(
    local_players: Res<LocalPlayers>,
    players: Query<(&Player, &Transform)>,
    mut cameras: Query<&mut Transform, (With<Camera>, Without<Player>)>,
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
