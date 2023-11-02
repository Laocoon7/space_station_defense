use bevy::{
    core_pipeline::{bloom::BloomSettings, clear_color::ClearColorConfig, tonemapping::Tonemapping},
    prelude::*,
    render::camera::ScalingMode,
};

use crate::{camera_plugin::components::tags::CameraTag, settings::AppSettings};

pub fn spawn_camera(mut commands: Commands) {
    let tile_size = AppSettings::lock().camera_settings().tile_size as f32;

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                is_active: true,
                hdr: true,
                ..Default::default()
            },
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::WindowSize(tile_size),
                ..Default::default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
            },
            transform: Transform::from_xyz(0.0, 0.0, 999.0),
            tonemapping: Tonemapping::TonyMcMapface,
            ..Default::default()
        },
        BloomSettings::default(),
        CameraTag,
    ));
}
