use bevy::prelude::*;

use crate::{builders::EnemyBuilder, resources::EnemyImages, types::EnemyType};

#[derive(Component, Clone)]
pub struct Spawner {
    pub timer: Timer,
    pub enemy_builder: EnemyBuilder,
}

impl Spawner {
    pub fn new(enemy_images: &EnemyImages, enemy_type: EnemyType, delay: f32) -> Self {
        Self {
            timer: Timer::from_seconds(delay, TimerMode::Repeating),
            enemy_builder: EnemyBuilder::new(enemy_images, enemy_type),
        }
    }
}
