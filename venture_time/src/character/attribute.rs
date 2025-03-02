use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Transform)]
pub struct Position {
    pub coords: Vec2,
}

impl Position {
    pub fn project(mut positionables: Query<(&mut Transform, &Position)>) {
        for (mut transform, position) in &mut positionables {
            transform.translation = position.coords.extend(0.)
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
    for mut hunger in hunger_query.iter_mut() {
        hunger.value = (hunger.value - hunger.decay_rate * time.delta_secs())
            .max(0.0)
            .min(100.0);
    }
}

/// System to regenerate stamina
pub fn regenerate_stamina(time: Res<Time>, mut stamina_query: Query<&mut Stamina>) {
    for mut stamina in stamina_query.iter_mut() {
        if stamina.current < stamina.max {
            stamina.current = (stamina.current + stamina.regen_rate * time.delta_secs())
                .min(stamina.max);
        }
    }
}

/// System to handle health regeneration based on hunger
pub fn health_regeneration(
    time: Res<Time>,
    mut query: Query<(&Hunger, &mut Health)>,
) {
    for (hunger, mut health) in query.iter_mut() {
        if hunger.value > 50.0 && health.current < health.max {
            // Only regenerate health if not too hungry
            let regen_rate = 1.0 * (hunger.value - 50.0) / 50.0; // Scale based on hunger
            health.current = (health.current + regen_rate * time.delta_secs()).min(health.max);
        }
    }
}
