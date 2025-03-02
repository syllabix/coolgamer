pub mod gabe {
    use bevy::{
        asset::{Assets, Handle},
        ecs::system::{Commands, Res, ResMut, Resource},
        image::Image,
        math::UVec2,
        sprite::TextureAtlasLayout,
    };

    use crate::character::asset;

    #[derive(Resource)]
    pub struct SpriteConfig {
        pub image: Handle<Image>,
        pub texture_atlas_layout: Handle<TextureAtlasLayout>,
        pub default_index: usize,
    }

    pub fn initialize(
        images: Res<asset::Images>,
        mut commands: Commands,
        mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let atlas_layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
        let layout = atlas_layouts.add(atlas_layout);
        let sprite_config = SpriteConfig {
            image: images.gabe.clone(),
            texture_atlas_layout: layout.clone(),
            default_index: 1,
        };
        commands.insert_resource(sprite_config);
    }
}
