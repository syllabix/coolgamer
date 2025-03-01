use bevy::{
    ecs::{
        component::Component,
        system::{Commands, Res},
    },
    sprite::Sprite,
    utils::default,
};

use super::{
    asset,
    attributes::{Health, Position},
};

#[derive(Component, Default)]
#[require(Position, Health)]
pub struct Player;

pub fn spawn(mut commands: Commands, sprites: Res<asset::Sprites>) {
    commands.spawn((
        Player,
        Sprite {
            image: sprites.gabe.clone(),
            ..default()
        },
    ));
}
