//! World module for Venture Time
//!
//! This module contains components and systems related to the game world,
//! including positioning, world boundaries, and environment interactions.
//!

use bevy::{
    ecs::{component::Component, system::Query},
    math::{Vec2, Vec3},
    transform::components::Transform,
};

/// The `Position` component handles the spatial representation of entities
/// in the game world, providing a layer of abstraction over Bevy's Transform
/// component for game-specific positioning logic.
#[derive(Component)]
#[require(Transform)]
pub struct Position {
    pub coords: Vec2,
    pub scale: Vec3,
}

impl Default for Position {
    fn default() -> Self {
        Self { coords: Vec2::ZERO, scale: Vec3::splat(1.) }
    }
}

/// Controls the rendering order of sprites.
/// Higher values will render on top of lower values.
#[derive(Component)]
#[derive(Default)]
pub struct ZIndex(pub i32);


/// Projects the `Position` component data onto the entity's `Transform` component.
///
/// This system synchronizes the abstract game position (stored in the `Position` component)
/// with Bevy's rendering system by updating the corresponding `Transform` component.
/// It handles both the 2D coordinates (projected to the XY plane with Z=0) and the scale
/// of each entity that has both components.
///
/// # Arguments
/// * `positionables` - Query for entities with both `Transform` and `Position` components
pub fn project_position(mut query: Query<(&mut Transform, &Position, Option<&ZIndex>)>) {
    for (mut transform, position, z_index) in &mut query {
        // Use the z_index value if available, otherwise default to 0
        let z = z_index.map_or(0.0, |z| z.0 as f32);
        transform.translation = position.coords.extend(z);
        transform.scale = position.scale;
    }
}
