use crate::world::{Position, ZIndex};
use bevy::{asset::Handle, ecs::{component::Component, error::Result, query::With, system::{Commands, Query, Res}}, image::Image, math::{Vec2, Vec3}, sprite::Sprite, window::{PrimaryWindow, Window}};

use super::Assets;

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct Prop;

// Z-index constants for different types of objects
const Z_GROUND: i32 = 0;
const Z_GRASS: i32 = 1;
const Z_ROCKS: i32 = 2;
const Z_BOARDS: i32 = 3;
const Z_BARRELS: i32 = 4;
const Z_CRATES: i32 = 5;
const Z_HOUSE: i32 = 6;
const Z_TREES: i32 = 7;

// Ground tile settings
const TILE_SIZE: f32 = 32.0;
const BASE_SCALE: f32 = 2.0;

pub fn spawn_level(
    mut commands: Commands,
    assets: Res<Assets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) -> Result {
    spawn_ground(&mut commands, &assets, &window_query)?;
    spawn_props(&mut commands, &assets, &window_query)?;
    Ok(())
}

fn spawn_ground(commands: &mut Commands, assets: &Assets, window_query: &Query<&Window, With<PrimaryWindow>>) -> Result {
    let window = window_query.single()?;
    
    // Calculate number of tiles needed to cover extended width (5x window width)
    let extended_width = window.width() * 5.0;
    let tiles_needed_x = (extended_width / TILE_SIZE).ceil() as i32;
    let start_x = -(window.width() / 2.0); // Keep starting at window edge
    let start_y = -(window.height() / 2.0);

    for x in 0..tiles_needed_x {
        // Spawn multiple grass decorations per tile
        for offset in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let grass_position = Vec2::new(
                TILE_SIZE.mul_add(offset, (x as f32).mul_add(TILE_SIZE, start_x)),
                3.5f32.mul_add(TILE_SIZE, start_y) + if offset % 0.5 == 0.0 { 2.0 } else { 0.0 },
            );
            
            let grass_handle = if (x as f32 + offset).floor() as i32 % 2 == 0 { 
                &assets.grass01 
            } else { 
                &assets.grass02 
            };
            
            commands.spawn((
                Prop,
                Sprite::from_image(grass_handle.clone()),
                Position {
                    coords: grass_position,
                    scale: Vec3::ONE * BASE_SCALE * (if offset % 0.5 == 0.0 { 0.7 } else { 0.6 }),
                },
                ZIndex(Z_GRASS),
            ));
        }

        // Spawn ground tiles
        for y in 0..4 {
            let tile_handle = match (x % 2, y) {
                // Column 1 pattern (x % 2 == 0)
                (0, 2 | 3) => &assets.ground_tile,
                (0, 1) => &assets.ground_tile1,
                (0, 0) => &assets.ground_tile2,
                // Column 2 pattern (x % 2 == 1)
                (1, 2 | 3) => &assets.ground_tile,
                (1, 1) => &assets.ground_tile3,
                (1, 0) => &assets.ground_tile4,
                // This case should never happen due to the modulo 2
                _ => &assets.ground_tile,
            };

            let position = Vec2::new(
                (x as f32).mul_add(TILE_SIZE, start_x),
                (y as f32).mul_add(TILE_SIZE, start_y),
            );

            commands.spawn((
                Ground,
                Sprite::from_image(tile_handle.clone()),
                Position {
                    coords: position,
                    scale: Vec3::ONE * BASE_SCALE,
                },
                ZIndex(Z_GROUND),
            ));
        }
    }

    Ok(())
}

fn spawn_props(commands: &mut Commands, assets: &Assets, window_query: &Query<&Window, With<PrimaryWindow>>) -> Result {
    let window = window_query.single()?;
    let window_width = window.width() * 5.0; // Extended to 5x window width
    let window_height = window.height();
    
    // Calculate base positions relative to window size
    let left_edge = -(window.width() / 2.0); // Start at left edge of view
    let right_edge = left_edge + window_width; // Extend far to the right
    let ground_level = -(window_height / 2.0);
    let prop_offset = TILE_SIZE * 4.0;

    // Create more sections for the extended width
    let section_width = window_width / 20.0; // More sections for better distribution
    let mut sections = Vec::new();
    let mut current = left_edge;
    
    while current < right_edge {
        sections.push((current, current + section_width));
        current += section_width;
    }

    // Spawn background trees (back layer) with more spacing
    for (i, (start, end)) in sections.iter().enumerate() {
        // Only spawn trees in every third section
        if i % 3 == 0 {
            let mid = (start + end) / 2.0;
            let tree_handle = if i % 2 == 0 { &assets.tree01 } else { &assets.tree02 };
            spawn_prop(commands, tree_handle, 
                Vec2::new(mid, prop_offset.mul_add(1.2, ground_level)), 
                Z_TREES, 
                BASE_SCALE * 1.3
            );
        }
    }
    
    // Spawn rocks throughout the landscape
    for (i, (start, end)) in sections.iter().enumerate() {
        if i % 2 == 0 {
            let x = start + (end - start) * 0.5;
            let rock_handle = match i % 6 {
                0 => &assets.rock01,
                1 => &assets.rock03,
                2 => &assets.rock04,
                3 => &assets.rock05,
                4 => &assets.rock06,
                _ => &assets.rock01,
            };
            spawn_prop(commands, rock_handle, 
                Vec2::new(x, prop_offset.mul_add(0.5, ground_level)), 
                Z_ROCKS, 
                BASE_SCALE * ((i % 3) as f32).mul_add(0.2, 0.8)
            );
        }
    }

    // Spawn barrels, crates, and boards in clusters
    for (i, (start, end)) in sections.iter().enumerate() {
        let mid = (start + end) / 2.0;
        
        // Create clusters of props in some sections
        if i % 4 == 0 {
            // Barrel cluster
            let barrel_handle = match i % 3 {
                0 => &assets.barrel01,
                1 => &assets.barrel02,
                _ => &assets.barrel03,
            };
            spawn_prop(commands, barrel_handle,
                Vec2::new(TILE_SIZE.mul_add(-0.5, mid), prop_offset.mul_add(0.4, ground_level)),
                Z_BARRELS,
                BASE_SCALE * 0.9
            );
            spawn_prop(commands, barrel_handle,
                Vec2::new(TILE_SIZE.mul_add(0.3, mid), prop_offset.mul_add(0.4, ground_level)),
                Z_BARRELS,
                BASE_SCALE * 0.8
            );
        } else if i % 4 == 2 {
            // Crate cluster
            let crate_handle = match i % 3 {
                0 => &assets.crate01,
                1 => &assets.crate02,
                _ => &assets.crate03,
            };
            spawn_prop(commands, crate_handle,
                Vec2::new(mid, prop_offset.mul_add(0.4, ground_level)),
                Z_CRATES,
                BASE_SCALE * 0.9
            );
            spawn_prop(commands, &assets.crate01,
                Vec2::new(TILE_SIZE.mul_add(0.6, mid), prop_offset.mul_add(0.4, ground_level)),
                Z_CRATES,
                BASE_SCALE * 0.8
            );
        }

        // Add scattered boards
        if i % 5 == 0 {
            let board_handle = match i % 4 {
                0 => &assets.board01,
                1 => &assets.board02,
                2 => &assets.board03,
                _ => &assets.board04,
            };
            spawn_prop(commands, board_handle,
                Vec2::new(TILE_SIZE.mul_add(0.2, mid), prop_offset.mul_add(0.3, ground_level)),
                Z_BOARDS,
                BASE_SCALE * 0.8
            );
        }
    }
    
    // Add houses spread throughout the level
    let house_positions = [-0.8, -0.4, 0.0, 0.4, 0.8]; // Relative positions along the extended width
    for pos in house_positions {
        let x = (window_width * (pos + 1.0)).mul_add(0.5, left_edge); // Distribute houses across the level
        let scale = if pos == 0.0 { BASE_SCALE * 1.8 } else { BASE_SCALE * 1.5 }; // Larger central house
        spawn_prop(commands, &assets.house, Vec2::new(x, prop_offset.mul_add(1.5, ground_level)), Z_HOUSE, scale);
    }

    Ok(())
}

fn spawn_prop(commands: &mut Commands, texture: &Handle<Image>, position: Vec2, z_index: i32, scale: f32) {
    commands.spawn((
        Prop,
        Position {
            coords: position,
            scale: Vec3::ONE * scale,
        },
        Sprite::from_image(texture.clone()),
        ZIndex(z_index),
    ));
}
