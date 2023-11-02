use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    window::{WindowResolution, WindowTheme},
};

use super::systems::set_window_icon;
use crate::settings::AppSettings;

pub struct WindowsPlugin;
impl Plugin for WindowsPlugin {
    fn build(&self, app: &mut App) {
        let app_settings = AppSettings::lock();
        let window_settings = app_settings.window_settings();

        app.add_plugins((DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(
                        window_settings.resolution.x as f32,
                        window_settings.resolution.y as f32,
                    ),
                    title: "Space Station Defense".to_string(),
                    resizable: false,
                    // decorations: false,
                    window_theme: Some(WindowTheme::Dark),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest())
            .set(LogPlugin {
                filter: log_filter(log_level()).to_string(),
                level: log_level(),
            }),));

        app.add_systems(Startup, set_window_icon);
    }
}

const fn log_level() -> Level {
    if cfg!(feature = "dev") {
        Level::INFO
    } else {
        Level::ERROR
    }
}

const fn log_filter(log_level: Level) -> &'static str {
    match log_level {
        Level::INFO => {
            "wgpu_hal=warn,gfx_backend_metal=warn,wgpu_core=warn,bevy_render=info,lain=debug,\
             bevy_render::render_resource::pipeline_cache=warn,sequence=debug,naga=warn"
        },
        _ => "",
    }
}
