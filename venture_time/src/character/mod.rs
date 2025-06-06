//! Character module for the Venture Time game.
//!
//! This module contains all character-related functionality including:
//! - Player character definition and controls
//! - Character attributes (health, movement, direction)
//! - Sprite animation systems
//! - Asset loading for character sprites
//!
//! The module is organized into submodules for better separation of concerns:
//! - `asset`: Character sprite asset definitions and loading
//! - `attribute`: Character attributes and properties
//! - `player`: Player-specific components and systems
//! - `sprite`: Sprite animation and rendering

use bevy::{
    app::{Plugin, Update},
    ecs::schedule::{IntoScheduleConfigs},
    state::{
        condition::in_state,
        state::OnEnter,
    },
};
use leafwing_input_manager::plugin::InputManagerPlugin;

use crate::game::GameState;

mod asset;
mod attribute;
mod player;
mod sprite;

pub use asset::Assets;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(InputManagerPlugin::<player::Action>::default())            
            .add_systems(
                OnEnter(GameState::LaunchGame),
                player::spawn,
            )
            .add_systems(
                Update,
                (
                    player::handle_input,
                    player::movement.after(player::handle_input),
                    player::jump_physics.after(player::handle_input),
                    player::camera_follow.after(player::movement),
                    sprite::animate_sprite.after(player::movement).after(player::jump_physics)
                )
                .run_if(in_state(GameState::Playing)),
            );
    }
}

// pub fn finalize_asset_initialization(mut next_state: ResMut<NextState<GameState>>) {
//     next_state.set(GameState::LaunchGame);
// }


