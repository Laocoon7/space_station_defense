use bevy::prelude::*;

use super::systems::spawn_camera;

pub struct CamerasPlugin;
impl Plugin for CamerasPlugin {
    fn build(&self, app: &mut App) { app.add_systems(Startup, spawn_camera); }
}
