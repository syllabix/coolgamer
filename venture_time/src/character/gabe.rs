use bevy::{
    ecs::{
        component::Component,
        system::{Commands, Res},
    }, math::Vec2, sprite::{Sprite, TextureAtlas},
};

use super::{
    attribute::{Health, Position}, sprite::gabe,
};

#[derive(Component, Default)]
#[require(
    Position(|| Position { coords: Vec2::ZERO }), 
    Health
)]
pub struct Player;

pub fn spawn(mut commands: Commands, gabe_sprite: Res<gabe::SpriteConfig>) {
    let sprite = Sprite {
        image: gabe_sprite.image.clone(),
        texture_atlas: Some(TextureAtlas {
            layout: gabe_sprite.texture_atlas_layout.clone(),
            index: gabe_sprite.default_index,
        }),
        ..Default::default()
    };
    commands.spawn((
        Player,
        sprite,        
    ));
}
