use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;

use crate::{
    builders::SpawnerBuilder,
    resources::{DifficultyTimer, EnemyImages},
};

pub fn up_difficulty(
    mut commands: Commands,
    enemy_images: Res<EnemyImages>,
    mut difficulty_timer: ResMut<DifficultyTimer>,
    time: Res<Time>,
) {
    difficulty_timer.tick(time.delta());
    if difficulty_timer.just_finished() {
        let mut rng = Pcg64::from_entropy();

        let mut spawner_builder = SpawnerBuilder::new(&enemy_images, rng.gen(), 2.0);
        spawner_builder.color(Color::ORANGE_RED).spawn(&mut commands, Vec2::new(500.0, 500.0));
    }
}
