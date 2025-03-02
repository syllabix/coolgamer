//! Character module for Venture Time
//!
//! This module contains character-related components, resources, and systems.
//! Characters include playable characters and NPCs that appear in the game.
//!
//! ## Submodules
//! - `gabe`: Contains Gabe character implementation

use asset::Images;
use bevy::{
    app::{Plugin, Update},
    ecs::{schedule::IntoSystemConfigs, system::ResMut},
    state::{
        condition::in_state,
        state::{NextState, OnEnter},
    },
};
use bevy_asset_loader::loading_state::{
    config::ConfigureLoadingState, LoadingState, LoadingStateAppExt,
};
use leafwing_input_manager::plugin::InputManagerPlugin;

use crate::game::GameState;

mod asset;
mod attribute;
mod player;
mod sprite;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(InputManagerPlugin::<player::Action>::default())
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading)
                    .load_collection::<Images>()
                    .continue_to_state(GameState::AssetInitializing),
            )
            .add_systems(
                OnEnter(GameState::AssetInitializing),
                (
                    sprite::gabe::initialize,
                    finalize_asset_initialization.after(sprite::gabe::initialize),
                ),
            )
            .add_systems(
                OnEnter(GameState::LaunchGame),
                (player::spawn, start_playing.after(player::spawn)),
            )
            .add_systems(
                Update,
                (
                    player::handle_input,
                    player::movement.after(player::handle_input),
                    sprite::animate_sprite.after(player::movement)
                )
                .run_if(in_state(GameState::Playing)),
            );
    }
}

pub fn finalize_asset_initialization(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::LaunchGame);
}

pub fn start_playing(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Playing);
}
