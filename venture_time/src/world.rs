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
#[derive(Component, Default)]
#[require(Transform)]
pub struct Position {
    pub coords: Vec2,
    pub scale: Vec3,
}

/// Projects the `Position` component data onto the entity's `Transform` component.
///
/// This system synchronizes the abstract game position (stored in the `Position` component)
/// with Bevy's rendering system by updating the corresponding `Transform` component.
/// It handles both the 2D coordinates (projected to the XY plane with Z=0) and the scale
/// of each entity that has both components.
///
/// # Arguments
/// * `positionables` - Query for entities with both `Transform` and `Position` components
pub fn project_position(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.coords.extend(0.);
        transform.scale = position.scale;
    }
}
