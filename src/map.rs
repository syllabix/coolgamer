use bevy::{color::Color, ecs::system::Commands, math::{Vec2, Vec3}, sprite::Sprite, transform::components::Transform, utils::default};


pub const MAP_SIZE: u32 = 41;
const GRID_WIDTH: f32 = 0.05;

pub fn setup(mut commands: Commands) {
    // Horizontal lines
    for i in 0..=MAP_SIZE {
        commands.spawn((
            Transform::from_translation(Vec3::new(0., i as f32 - MAP_SIZE as f32 / 2., 0.)),
            Sprite {
                color: Color::srgb(0.27, 0.27, 0.27),
                custom_size: Some(Vec2::new(MAP_SIZE as f32, GRID_WIDTH)),
                ..default()
            },
        ));
    }

    // Vertical lines
    for i in 0..=MAP_SIZE {
        commands.spawn((
            Transform::from_translation(Vec3::new(i as f32 - MAP_SIZE as f32 / 2., 0., 0.)),
            Sprite {
                color: Color::srgb(0.27, 0.27, 0.27),
                custom_size: Some(Vec2::new(GRID_WIDTH, MAP_SIZE as f32)),
                ..default()
            },
        ));
    }
}