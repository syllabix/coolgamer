use std::time::Duration;

use bevy::{
    ecs::component::Component,
    time::{Time, Timer, TimerMode},
    sprite::{Sprite, TextureAtlas},
    ecs::system::{Query, Res},
};

use crate::character::attribute::{Movement, Direction};

#[derive(Component)]
pub struct AnimationConfig {
    fps: u8,
    frame_timer: Timer,
    last_sprite_index: usize,
    first_sprite_index: usize,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            fps,
            frame_timer: Self::timer_from_fps(fps),
            last_sprite_index: last,
            first_sprite_index: first,
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Repeating)
    }
    
    pub fn reset(&mut self) {
        self.frame_timer.reset();
    }
}

/// System that animates sprites based on movement
pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&Movement, &mut AnimationConfig, &mut Sprite)>,
) {
    for (movement, mut animation, mut sprite) in query.iter_mut() {
        // Update sprite flip based on movement direction
        sprite.flip_x = match movement.direction {
            Direction::Right => false,
            Direction::Left => true,
        };
        
        // Check if the entity is moving
        let is_moving = movement.velocity.length_squared() > 0.01;
        
        // Get the texture atlas from the sprite
        let Some(texture_atlas) = &mut sprite.texture_atlas else {
            continue;
        };
        
        if is_moving {
            // Only tick the timer when moving
            animation.frame_timer.tick(time.delta());
            
            // Update the sprite index when the timer finishes
            if animation.frame_timer.just_finished() {
                // Cycle to the next sprite
                let next_index = if texture_atlas.index >= animation.last_sprite_index {
                    animation.first_sprite_index
                } else {
                    texture_atlas.index + 1
                };
                
                texture_atlas.index = next_index;
            }
        } else {
            // Reset to the first frame when not moving
            texture_atlas.index = animation.first_sprite_index;
            animation.reset();
        }
    }
}

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
        pub first_index: usize,
        pub last_index: usize,
        pub fps: u8
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
            first_index: 0,
            last_index: 6,
            fps: 20
        };
        commands.insert_resource(sprite_config);
    }
}
