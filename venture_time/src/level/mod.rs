use bevy::prelude::*;
use crate::game::GameState;

mod asset;
mod one;

pub use asset::Assets;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            one::spawn_level,
        );
    }
}
