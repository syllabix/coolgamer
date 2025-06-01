use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Component, Clone, Copy)]
pub struct Movement {
    pub speed: f32,
    pub velocity: Vec2,
    pub direction: Direction,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            speed: 1.0,
            velocity: Default::default(),
            direction: Direction::Right,
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct Jump {
    pub is_jumping: bool,
    pub jump_velocity: f32,
    pub gravity: f32,
    pub jump_height: f32,
    pub ground_level: f32,
}

impl Default for Jump {
    fn default() -> Self {
        Self {
            is_jumping: false,
            jump_velocity: 10.0,
            gravity: 0.5,
            jump_height: 0.0,
            ground_level: 0.0,
        }
    }
}