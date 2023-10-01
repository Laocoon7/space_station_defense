use bevy::prelude::*;

use super::bundles::EnemyBundle;
use crate::{resources::EnemyImages, types::EnemyType};

#[derive(Clone)]
pub struct EnemyBuilder {
    bundle: EnemyBundle,
}

impl EnemyBuilder {
    pub fn new(enemy_images: &EnemyImages, enemy_type: EnemyType) -> Self {
        Self {
            bundle: enemy_type.bundle(enemy_images),
        }
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.bundle.dynamic_object.static_object.sprite_bundle.sprite.color = color;
        self
    }

    pub fn spawn(&self, commands: &mut Commands, position: Vec2) {
        let mut bundle = self.bundle.clone();
        let translation = &mut bundle.dynamic_object.static_object.sprite_bundle.transform.translation;
        *translation = position.extend(translation.z);

        commands.spawn(bundle);
    }
}
