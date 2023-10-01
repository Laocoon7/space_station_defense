use bevy::prelude::*;

use crate::components::Spawner;

pub fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut q_spawners: Query<(&Transform, &mut Spawner)>,
) {
    for (transform, mut spawner) in q_spawners.iter_mut() {
        spawner.timer.tick(time.delta());
        if spawner.timer.just_finished() {
            spawner.enemy_builder.spawn(&mut commands, transform.translation.truncate());
        }
    }
}
