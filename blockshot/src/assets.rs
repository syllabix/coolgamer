use bevy::{asset::Handle, ecs::system::Resource, image::Image};
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "bullet.png")]
    pub bullet: Handle<Image>
}