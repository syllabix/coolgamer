//! Game state management and scoring system.
//!
//! This module handles the core game state including:
//! - Score tracking for both players
//! - Point scoring detection when ball goes past paddles
//! - Ball reset logic after points
//! - Events for communicating scoring between systems
//!
//! The scoring system uses Bevy's event system to decouple point detection
//! from score updates. The Score resource maintains the current game state.

use bevy::{ecs::{event::{Event, EventReader, EventWriter}, query::With, system::{Query, ResMut, Resource}}, math::Vec2, window::Window};

use crate::components::{Ball, Position, Velocity};


pub enum Scorer {
    Player,
    Opponent
}

#[derive(Event)]
pub struct PointScored {
    by: Scorer
}

#[derive(Resource, Default)]
pub struct Score {
    pub player: usize,
    pub opponent: usize,
}

pub fn detect_scoring(
    mut ball: Query<&mut Position, With<Ball>>,
    window: Query<&Window>,
    mut events: EventWriter<PointScored>
) {
    if let Ok(window) = window.get_single() {
        let width = window.resolution.width();

        if let Ok(ball) = ball.get_single_mut() {
            if ball.coords.x > width / 2. {
                events.send(PointScored { by: Scorer::Opponent });
            } else if ball.coords.x < -width / 2. {
                events.send(PointScored { by: Scorer::Player });
            }
        }
    }
}

pub fn reset_ball(
    mut ball: Query<(&mut Position, &mut Velocity), With<Ball>>,
    mut events: EventReader<PointScored>,
  ) {
    for scored in events.read() {
      if let Ok((mut position, mut velocity)) = ball.get_single_mut() {
        match scored.by {
          Scorer::Opponent => {
            position.coords = Vec2::new(0., 0.);
            velocity.direction = Vec2::new(-1., 1.);
          }
          Scorer::Player => {
            position.coords = Vec2::new(0., 0.);
            velocity.direction = Vec2::new(1., 1.);
          }
        }
      }
    }
  }
  
  pub fn update_score(mut score: ResMut<Score>, mut events: EventReader<PointScored>) {
    for scored in events.read() {
      match scored.by {
        Scorer::Opponent => score.opponent += 1,
        Scorer::Player => score.player += 1,
      }
    }
  }