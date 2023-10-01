use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{distributions::Standard, prelude::Distribution};

use crate::{
    builders::bundles::{DynamicObject, EnemyBundle},
    components::{Health, MovementType},
    resources::EnemyImages,
};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnemyType {
    Mercury,
    Mars,
    Venus,
    Earth,
}

impl EnemyType {
    pub fn name(self) -> &'static str {
        match self {
            EnemyType::Mercury => "Mercury",
            EnemyType::Mars => "Mars",
            EnemyType::Venus => "Venus",
            EnemyType::Earth => "Earth",
        }
    }

    pub fn bundle(self, enemy_images: &EnemyImages) -> EnemyBundle {
        EnemyBundle {
            dynamic_object: DynamicObject::new(
                self.name(),
                enemy_images.get_handle(self),
                Color::WHITE,
                Vec2::ZERO,
                self.collider(),
                MovementType::Location(Vec2::ZERO, self.speed()),
            ),
            ..Default::default()
        }
    }

    pub fn collider(self) -> Collider {
        match self {
            EnemyType::Mercury => Collider::ball(4.0),
            EnemyType::Mars => Collider::ball(8.0),
            EnemyType::Venus => Collider::ball(12.0),
            EnemyType::Earth => Collider::ball(16.0),
        }
    }

    pub fn speed(self) -> f32 {
        match self {
            EnemyType::Mercury => 100.0,
            EnemyType::Mars => 50.0,
            EnemyType::Venus => 45.0,
            EnemyType::Earth => 25.0,
        }
    }

    pub fn health(self) -> Health {
        match self {
            EnemyType::Mercury => Health {
                current: 10,
                max: 10,
            },
            EnemyType::Mars => Health {
                current: 10,
                max: 10,
            },
            EnemyType::Venus => Health {
                current: 10,
                max: 10,
            },
            EnemyType::Earth => Health {
                current: 10,
                max: 10,
            },
        }
    }
}

impl Distribution<EnemyType> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> EnemyType {
        match rng.gen_range(0..4) {
            0 => EnemyType::Mercury,
            1 => EnemyType::Mars,
            2 => EnemyType::Venus,
            3 => EnemyType::Earth,
            _ => unreachable!(),
        }
    }
}
