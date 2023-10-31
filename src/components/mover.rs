use bevy::prelude::*;

use crate::types::Polar;

#[derive(Clone, Copy)]
pub enum MovementType {
    Location(Vec2, f32),
    Direction(Vec2, f32),
    Theta(f32),
}

#[derive(Component, Clone, Copy)]
pub struct Mover {
    pub movement_type: MovementType,
}

impl Mover {
    pub fn apply(self, transform: &mut Transform, time: &Time) {
        match self.movement_type {
            MovementType::Location(location, speed) => {
                let direction = (location - transform.translation.truncate()).normalize_or_zero();
                transform.translation += (direction * time.delta_seconds() * speed).extend(0.0)
            },
            MovementType::Direction(direction, speed) => {
                transform.translation += (direction * time.delta_seconds() * speed).extend(0.0)
            },
            MovementType::Theta(theta) => {
                let mut polar = Polar::from_vec2(transform.translation.truncate());
                polar.theta += theta;
                transform.translation = polar.to_vec2().extend(transform.translation.z);
            },
        }
    }
}

impl Default for Mover {
    fn default() -> Self {
        Self {
            movement_type: MovementType::Location(Vec2::ZERO, 50.0),
        }
    }
}
