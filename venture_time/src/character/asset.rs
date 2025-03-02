use bevy::{
    asset::Handle,
    ecs::system::Resource,
    image::Image,
};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct Images {
    #[asset(path = "chars/gabe/gabe-running.png")]
    pub gabe: Handle<Image>,
}
