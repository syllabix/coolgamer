//! Scoreboard module for displaying and updating game scores.
//!
//! This module handles the visual representation of the game score, including:
//! - Separate text components for player and opponent scores
//! - Score display positioning and styling
//! - Score update system that responds to score changes
//!
//! The scoreboard uses Bevy's UI system with absolute positioning and
//! custom styling. It observes the Score resource and updates the display
//! automatically when scores change.

use bevy::{
    color::Color, ecs::{
        change_detection::DetectChanges,
        component::Component,
        query::{With, Without},
        system::{Commands, Query, Res},
    }, text::{JustifyText, TextColor, TextFont, TextLayout}, ui::{widget::Text, Node, PositionType, Val}, utils::default
};

use crate::game::Score;

#[derive(Component)]
pub struct PlayerScore;

#[derive(Component)]
pub struct OpponentScore;

pub fn update(
    mut player_score: Query<&mut Text, With<PlayerScore>>,
    mut opponent_score: Query<&mut Text, (With<OpponentScore>, Without<PlayerScore>)>,
    score: Res<Score>,
) {
    if score.is_changed() {
        if let Ok(mut player_score) = player_score.get_single_mut() {
            player_score.0 = score.player.to_string();
        }

        if let Ok(mut opponent_score) = opponent_score.get_single_mut() {
            opponent_score.0 = score.opponent.to_string();
        }
    }
}

pub fn spawn(mut commands: Commands) {
    commands.spawn((
        PlayerScore,
        Text::new("0"),
        TextFont {
            font_size: 72.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        },
    ));

    commands.spawn((
        OpponentScore,
        Text::new("0"),
        TextFont {
            font_size: 72.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        },
    ));
}
