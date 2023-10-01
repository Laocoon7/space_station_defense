use bevy::prelude::*;

use super::{bundles::SpawnerBundle, EnemyBuilder};
use crate::{resources::EnemyImages, types::EnemyType};

pub struct SpawnerBuilder {
    bundle: SpawnerBundle,
}

impl SpawnerBuilder {
    pub fn new(enemy_images: &EnemyImages, enemy_type: EnemyType, delay: f32) -> Self {
        Self {
            bundle: SpawnerBundle::new(enemy_images, enemy_type, delay),
        }
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.bundle.spawner.enemy_builder.color(color);
        self
    }

    pub fn enemy_type(
        &mut self,
        enemy_images: &EnemyImages,
        enemy_type: EnemyType,
        color: Color,
    ) -> &mut Self {
        self.bundle.spawner.enemy_builder = EnemyBuilder::new(enemy_images, enemy_type);
        self.color(color);
        self
    }

    pub fn spawn(&self, commands: &mut Commands, position: Vec2) {
        let mut bundle = self.bundle.clone();
        let translation = &mut bundle.transform.translation;
        *translation = position.extend(translation.z);

        commands.spawn(bundle);
    }
}
