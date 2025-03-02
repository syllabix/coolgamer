use crate::world::{Position, ZIndex};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::seq::SliceRandom;

use super::Assets;

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct Prop;

// Z-index constants for different types of objects
const Z_GROUND: i32 = 0;
const Z_GRASS: i32 = 1;
const Z_ROCKS: i32 = 2;
const Z_FLOWERS: i32 = 3;
const Z_HOUSE: i32 = 4;
const Z_TREES: i32 = 5;

// Ground tile settings
const TILE_SIZE: f32 = 32.0;
const GROUND_HEIGHT: i32 = 4;
const BASE_SCALE: f32 = 2.0;

pub fn spawn_level(
    mut commands: Commands,
    assets: Res<Assets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    spawn_ground(&mut commands, &assets, &window_query);
    spawn_props(&mut commands, &assets, &window_query);
}

fn spawn_ground(commands: &mut Commands, assets: &Assets, window_query: &Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();
    
    // Calculate number of tiles needed to cover window width
    let tiles_needed_x = (window.width() / TILE_SIZE).ceil() as i32;
    let start_x = -(window.width() / 2.0);
    let start_y = -(window.height() / 2.0); // Position at bottom of window

    for x in 0..tiles_needed_x {
        // Spawn multiple grass decorations per tile
        for offset in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let grass_position = Vec2::new(
                start_x + (x as f32 * TILE_SIZE) + (TILE_SIZE * offset),
                start_y + (3.5 * TILE_SIZE) + if offset % 0.5 == 0.0 { 2.0 } else { 0.0 },
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
                (0, 2) | (0, 3) => &assets.ground_tile,
                (0, 1) => &assets.ground_tile1,
                (0, 0) => &assets.ground_tile2,
                // Column 2 pattern (x % 2 == 1)
                (1, 2) | (1, 3) => &assets.ground_tile,
                (1, 1) => &assets.ground_tile3,
                (1, 0) => &assets.ground_tile4,
                // This case should never happen due to the modulo 2
                _ => &assets.ground_tile,
            };

            let position = Vec2::new(
                start_x + (x as f32 * TILE_SIZE),
                start_y + (y as f32 * TILE_SIZE),
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
}

fn spawn_props(commands: &mut Commands, assets: &Assets, window_query: &Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();
    let window_width = window.width() * 2.0; // Extended level width
    let window_height = window.height();
    
    // Calculate base positions relative to window size
    let left_edge = -window_width * 0.5;
    let right_edge = window_width * 0.5;
    let ground_level = -(window_height / 2.0);
    let prop_offset = TILE_SIZE * 4.0;

    // Create sections for prop placement
    let section_width = window_width / 8.0;
    let sections = [
        (left_edge, left_edge + section_width),             // Far left
        (left_edge + section_width, left_edge + section_width * 2.0),  // Left
        (left_edge + section_width * 2.0, 0.0),            // Center left
        (0.0, right_edge - section_width * 2.0),           // Center right
        (right_edge - section_width * 2.0, right_edge - section_width), // Right
        (right_edge - section_width, right_edge),          // Far right
    ];

    // Spawn background trees (back layer) across sections
    for (start, end) in sections {
        let mid = (start + end) / 2.0;
        spawn_prop(commands, &assets.tree01, Vec2::new(start + section_width * 0.2, ground_level + prop_offset * 1.2), Z_TREES, BASE_SCALE * 1.4);
        spawn_prop(commands, &assets.tree02, Vec2::new(mid, ground_level + prop_offset * 1.1), Z_TREES, BASE_SCALE * 1.2);
        spawn_prop(commands, &assets.tree01, Vec2::new(end - section_width * 0.2, ground_level + prop_offset * 1.3), Z_TREES, BASE_SCALE * 1.3);
    }
    
    // Spawn rocks throughout the landscape
    for (start, end) in sections {
        let positions = [0.2, 0.5, 0.8]; // Relative positions within each section
        for pos in positions {
            let x = start + (end - start) * pos;
            let rock_handle = if pos > 0.5 { &assets.rock01 } else { &assets.rock03 };
            spawn_prop(commands, rock_handle, 
                Vec2::new(x, ground_level + prop_offset * 0.5), 
                Z_ROCKS, 
                BASE_SCALE * (0.8 + pos * 0.4) // Vary rock sizes
            );
        }
    }
    
    // Spawn flowers and grass clusters
    for (start, end) in sections {
        let cluster_positions = [0.1, 0.3, 0.5, 0.7, 0.9]; // More positions for dense vegetation
        for pos in cluster_positions {
            let x = start + (end - start) * pos;
            
            // Flower cluster
            spawn_prop(commands, &assets.flowers, 
                Vec2::new(x - TILE_SIZE, ground_level + prop_offset * 0.3), 
                Z_FLOWERS, BASE_SCALE * 0.8
            );
            spawn_prop(commands, &assets.flower01, 
                Vec2::new(x + TILE_SIZE, ground_level + prop_offset * 0.3), 
                Z_FLOWERS, BASE_SCALE * 0.7
            );
            
            // Grass cluster
            spawn_prop(commands, &assets.grass01, 
                Vec2::new(x, ground_level + prop_offset * 0.2), 
                Z_GRASS, BASE_SCALE * 0.7
            );
            spawn_prop(commands, &assets.grass02, 
                Vec2::new(x + TILE_SIZE * 0.5, ground_level + prop_offset * 0.25), 
                Z_GRASS, BASE_SCALE * 0.6
            );
        }
    }
    
    // Add houses as points of interest
    spawn_prop(commands, &assets.house, Vec2::new(-section_width * 2.0, ground_level + prop_offset * 1.5), Z_HOUSE, BASE_SCALE * 1.5);
    spawn_prop(commands, &assets.house, Vec2::new(section_width * 2.0, ground_level + prop_offset * 1.5), Z_HOUSE, BASE_SCALE * 1.5);
    spawn_prop(commands, &assets.house, Vec2::new(0.0, ground_level + prop_offset * 1.5), Z_HOUSE, BASE_SCALE * 1.8); // Larger central house
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
