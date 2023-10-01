use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;

use crate::{
    components::{tags::SpawnerTag, MovementType, Mover, Spawner},
    resources::EnemyImages,
    types::{EnemyType, ScreenLayer},
};

#[derive(Bundle, Clone)]
pub struct SpawnerBundle {
    pub transform: Transform,
    pub mover: Mover,
    pub spawner: Spawner,
    pub spawner_tag: SpawnerTag,
}

impl SpawnerBundle {
    pub fn new(enemy_images: &EnemyImages, enemy_type: EnemyType, delay: f32) -> Self {
        let mut rng = Pcg64::from_entropy();

        Self {
            transform: Transform::from_translation(Vec2::ZERO.extend(ScreenLayer::Background.into())),
            mover: Mover {
                movement_type: MovementType::Theta(rng.gen_range(10.0..=45.0)),
            },
            spawner: Spawner::new(enemy_images, enemy_type, delay),
            spawner_tag: SpawnerTag,
        }
    }
}
