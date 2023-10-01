use bevy::prelude::*;

use super::{DifficultyTimer, EnemyImages, SpaceStationImage};

pub struct ResourcesPlugin;
impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpaceStationImage>();
        app.init_resource::<EnemyImages>();
        app.init_resource::<DifficultyTimer>();
    }
}
