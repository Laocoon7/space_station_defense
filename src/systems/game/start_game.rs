use bevy::prelude::*;

use crate::{
    builders::SpawnerBuilder,
    functions::spawn_space_station,
    resources::{DifficultyTimer, EnemyImages, SpaceStationImage},
    types::{EnemyType, GameState},
};

pub fn start_game(
    mut commands: Commands,
    space_station_image: Res<SpaceStationImage>,
    enemy_images: Res<EnemyImages>,
    mut difficulty_timer: ResMut<DifficultyTimer>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    spawn_space_station(&mut commands, &space_station_image);

    let mut spawner_builder = SpawnerBuilder::new(&enemy_images, EnemyType::Mars, 2.0);
    spawner_builder.color(Color::ORANGE_RED).spawn(&mut commands, Vec2::new(500.0, 500.0));
    // spawner_builder.enemy_type(&enemy_images, EnemyType::Mercury, Color::RED).spawn(&mut
    // commands, Vec2::new(500.0, -500.0)); spawner_builder.enemy_type(&enemy_images,
    // EnemyType::Venus, Color::WHITE).spawn(&mut commands, Vec2::new(-500.0, 500.0));
    // spawner_builder.enemy_type(&enemy_images, EnemyType::Mercury, Color::YELLOW).spawn(&mut
    // commands, Vec2::new(-500.0, -500.0));

    difficulty_timer.reset();
    difficulty_timer.unpause();

    game_state.set(GameState::Playing);
}
