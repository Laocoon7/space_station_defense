use bevy::{prelude::*, utils::HashMap};

use crate::types::EnemyType;

#[derive(Resource)]
pub struct EnemyImages(pub HashMap<EnemyType, Handle<Image>>);

impl EnemyImages {
    pub fn get_handle(&self, enemy_type: EnemyType) -> Handle<Image> {
        self.0
            .get(&enemy_type)
            .cloned()
            .expect(format!("Failed to get image for {:?}", enemy_type).as_str())
    }
}

impl FromWorld for EnemyImages {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().expect("Failed to get asset server");
        let mut hash_map = HashMap::new();

        hash_map.insert(
            EnemyType::Mercury,
            asset_server.load("sprites/enemies/8x8_mercury.png"),
        );
        hash_map.insert(
            EnemyType::Mars,
            asset_server.load("sprites/enemies/16x16_mars.png"),
        );
        hash_map.insert(
            EnemyType::Venus,
            asset_server.load("sprites/enemies/24x24_venus.png"),
        );
        hash_map.insert(
            EnemyType::Earth,
            asset_server.load("sprites/enemies/32x32_earth.png"),
        );

        Self(hash_map)
    }
}
