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

/// Health component for characters
#[derive(Component, Debug, Clone, Copy)]
pub struct Health {
    /// Current health points
    pub current: f32,
    /// Maximum health points
    pub max: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
        }
    }
}

/// Strength attribute affects damage dealt
#[derive(Component, Debug, Clone, Copy)]
pub struct Strength {
    /// Base strength value
    pub value: f32,
}

impl Default for Strength {
    fn default() -> Self {
        Self { value: 10.0 }
    }
}

/// Defense attribute reduces damage taken
#[derive(Component, Debug, Clone, Copy)]
pub struct Defense {
    /// Base defense value
    pub value: f32,
}

impl Default for Defense {
    fn default() -> Self {
        Self { value: 5.0 }
    }
}

/// Speed affects movement and action rates
#[derive(Component, Debug, Clone, Copy)]
pub struct Speed {
    /// Base speed value
    pub value: f32,
}

impl Default for Speed {
    fn default() -> Self {
        Self { value: 5.0 }
    }
}

/// Stamina for running, attacking, and other actions
#[derive(Component, Debug, Clone, Copy)]
pub struct Stamina {
    /// Current stamina points
    pub current: f32,
    /// Maximum stamina points
    pub max: f32,
    /// Rate at which stamina regenerates
    pub regen_rate: f32,
}

impl Default for Stamina {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
            regen_rate: 5.0,
        }
    }
}

/// Hunger decreases over time and affects health regeneration
#[derive(Component, Debug, Clone, Copy)]
pub struct Hunger {
    /// Current hunger value (0 = starving, 100 = full)
    pub value: f32,
    /// Rate at which hunger decreases
    pub decay_rate: f32,
}

impl Default for Hunger {
    fn default() -> Self {
        Self {
            value: 100.0,
            decay_rate: 0.5,
        }
    }
}

/// Inventory capacity based on strength
#[derive(Component, Debug, Clone, Copy)]
pub struct InventoryCapacity {
    /// Maximum weight the character can carry
    pub max_weight: f32,
}

impl Default for InventoryCapacity {
    fn default() -> Self {
        Self { max_weight: 50.0 }
    }
}

/// System to update hunger over time
pub fn update_hunger(time: Res<Time>, mut hunger_query: Query<&mut Hunger>) {
    for mut hunger in &mut hunger_query {
        hunger.value = hunger.decay_rate.mul_add(-time.delta_secs(), hunger.value).clamp(0.0, 100.0);
    }
}

/// System to regenerate stamina
pub fn regenerate_stamina(time: Res<Time>, mut stamina_query: Query<&mut Stamina>) {
    for mut stamina in &mut stamina_query {
        if stamina.current < stamina.max {
            stamina.current =
                stamina.regen_rate.mul_add(time.delta_secs(), stamina.current).min(stamina.max);
        }
    }
}

/// System to handle health regeneration based on hunger
pub fn health_regeneration(time: Res<Time>, mut query: Query<(&Hunger, &mut Health)>) {
    for (hunger, mut health) in &mut query {
        if hunger.value > 50.0 && health.current < health.max {
            // Only regenerate health if not too hungry
            let regen_rate = 1.0 * (hunger.value - 50.0) / 50.0; // Scale based on hunger
            health.current = regen_rate.mul_add(time.delta_secs(), health.current).min(health.max);
        }
    }
}
