//! Character module for Venture Time
//!
//! This module contains character-related components, resources, and systems.
//! Characters include playable characters and NPCs that appear in the game.
//!
//! ## Submodules
//! - `gabe`: Contains Gabe character implementation

use bevy::{app::Plugin, state::state::OnEnter};

use crate::game_state::GameState;


mod gabe;
mod attributes;
pub mod asset;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(OnEnter(GameState::Playing), gabe::spawn);
    }
}