use bevy::prelude::*;
use bevy_rapier2d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use camera_plugin::CamerasPlugin;
use settings::AppSettings;
use windows_plugin::WindowsPlugin;

pub mod camera_plugin;
pub mod objects_plugin;
pub mod settings;
pub mod windows_plugin;

fn main() {
    let tile_size = AppSettings::lock().camera_settings().tile_size as f32;

    let mut app = App::new();

    app.insert_resource(RapierConfiguration {
        gravity: Vec2::ZERO,
        ..Default::default()
    });

    app.add_plugins((
        // Default
        WindowsPlugin,
        // External
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(tile_size),
        #[cfg(feature = "dev")]
        RapierDebugRenderPlugin::default(),
        // Ours
        CamerasPlugin,
        fortress::prelude::AnimationPlugin,
    ));

    app.run();
}
