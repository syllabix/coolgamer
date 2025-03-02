//! Character module for Venture Time
//!
//! This module contains character-related components, resources, and systems.
//! Characters include playable characters and NPCs that appear in the game.
//!
//! ## Submodules
//! - `gabe`: Contains Gabe character implementation

use asset::Images;
use bevy::{app::Plugin, ecs::{schedule::IntoSystemConfigs, system::ResMut}, state::state::{NextState, OnEnter}};
use bevy_asset_loader::loading_state::{
    config::ConfigureLoadingState, LoadingState, LoadingStateAppExt,
};

use crate::game_state::GameState;

mod asset;
mod attribute;
mod sprite;
mod gabe;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .load_collection::<Images>()
                .continue_to_state(GameState::AssetInitializing),
        )
        .add_systems(
            OnEnter(GameState::AssetInitializing), (
                sprite::gabe::initialize,
                finalize_asset_initialization.after(sprite::gabe::initialize),
            ),
        )
        .add_systems(OnEnter(GameState::Playing), gabe::spawn);
    }
}


pub fn finalize_asset_initialization(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Playing);
}