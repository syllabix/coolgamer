use bevy::{
    asset::Handle,
    ecs::{resource::Resource, system::Res},
    image::{Image, TextureAtlasLayout},
};
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct Assets {
    #[asset(key = "venture_guy")]
    pub venture_guy: Handle<Image>,
    #[asset(key = "venture_guy.player_sheet")]
    pub venture_guy_layout: Handle<TextureAtlasLayout>,
    // #[asset(key = "venture_girl")]
    // pub venture_girl: Handle<Image>,
    // #[asset(key = "venture_girl.player_sheet")]
    // pub venture_girl_layout: Handle<TextureAtlasLayout>,
}
