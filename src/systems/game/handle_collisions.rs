use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{
    tags::{EnemyTag, SpaceStationTag},
    Health,
};

pub fn handle_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut q_space_station: Query<(Entity, &mut Health), With<SpaceStationTag>>,
    q_enemies: Query<Entity, With<EnemyTag>>,
    // q_projectiles: Query<Entity, With<ProjectileTag>>,
) {
    let (space_station_entity, mut space_station_health) = q_space_station.single_mut();

    for collision_event in collision_events.iter() {
        info!("Collision event: {:?}", &collision_event);

        match collision_event {
            CollisionEvent::Started(entity1, entity2, _flags) => {
                // Handle projectiles hitting the enemies

                // Handle enemies hitting the space station
                if let Some(other) = if *entity1 == space_station_entity {
                    Some(*entity2)
                } else if *entity2 == space_station_entity {
                    Some(*entity1)
                } else {
                    None
                } {
                    if q_enemies.contains(other) {
                        space_station_health.current -= 1;
                        commands.entity(other).despawn_recursive();
                        info!("Health: {}", space_station_health.current);
                    }
                }
            },
            _ => (),
        }
    }
}
