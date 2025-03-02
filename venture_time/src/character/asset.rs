use bevy::prelude::*;
use bevy::{
    asset::Handle,
    ecs::system::{Commands, ResMut, Resource},
    image::Image,
    sprite::TextureAtlasLayout,
};
use bevy_asset_loader::asset_collection::AssetCollection;

use crate::game_state::GameState;

#[derive(AssetCollection, Resource)]
pub struct Images {
    #[asset(path = "chars/gabe/gabe-running.png")]
    pub gabe: Handle<Image>,
}
