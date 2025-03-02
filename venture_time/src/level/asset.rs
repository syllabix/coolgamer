use bevy::{
    asset::Handle,
    ecs::system::Resource,
    image::Image,
};
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct Assets {
    // Ground tiles
    #[asset(path = "tiles/generic-rpg-Slice.png")]
    pub ground_tile: Handle<Image>,
    #[asset(path = "tiles/generic-rpg-tile01.png")]
    pub ground_tile1: Handle<Image>,
    #[asset(path = "tiles/generic-rpg-tile02.png")]
    pub ground_tile2: Handle<Image>,
    #[asset(path = "tiles/generic-rpg-tile03.png")]
    pub ground_tile3: Handle<Image>,
    #[asset(path = "tiles/generic-rpg-tile04.png")]
    pub ground_tile4: Handle<Image>,

    // Props
    #[asset(path = "props/generic-rpg-tree01.png")]
    pub tree01: Handle<Image>,
    #[asset(path = "props/generic-rpg-tree02.png")]
    pub tree02: Handle<Image>,
    #[asset(path = "props/generic-rpg-rock01.png")]
    pub rock01: Handle<Image>,
    #[asset(path = "props/generic-rpg-rock03.png")]
    pub rock03: Handle<Image>,
    #[asset(path = "props/generic-rpg-flowers.png")]
    pub flowers: Handle<Image>,
    #[asset(path = "props/generic-rpg-flower01.png")]
    pub flower01: Handle<Image>,
    #[asset(path = "props/generic-rpg-grass01.png")]
    pub grass01: Handle<Image>,
    #[asset(path = "props/generic-rpg-grass02.png")]
    pub grass02: Handle<Image>,
    #[asset(path = "props/generic-rpg-house-inn.png")]
    pub house: Handle<Image>,
} 