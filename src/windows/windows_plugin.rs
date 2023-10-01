use std::io::Cursor;

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    window::{PrimaryWindow, WindowResolution, WindowTheme},
    winit::WinitWindows,
};
use bevy_rapier2d::{
    prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use winit::window::Icon;

use crate::settings::AppSettings;

pub struct WindowsPlugin;
impl Plugin for WindowsPlugin {
    fn build(&self, app: &mut App) {
        let app_settings = AppSettings::lock();
        let window_settings = app_settings.window_settings();
        let camera_settings = app_settings.camera_settings();

        app.insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            // physics_pipeline_active: true,
            // query_pipeline_active: true,
            // timestep_mode: todo!(),
            // scaled_shape_subdivision: todo!(),
            // force_update_from_transform_changes: todo!(),
            ..Default::default()
        });

        app.add_plugins((
            DefaultPlugins
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
                    filter: Self::log_filter(Self::log_level()).to_string(),
                    level: Self::log_level(),
                }),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(camera_settings.tile_size as f32),
        ));

        #[cfg(feature = "dev")]
        app.add_plugins(RapierDebugRenderPlugin::default());

        app.add_systems(Startup, Self::set_window_icon);
    }
}

impl WindowsPlugin {
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

    fn set_window_icon(windows: NonSend<WinitWindows>, primary_windows: Query<Entity, With<PrimaryWindow>>) {
        let primary_entity = primary_windows.single();
        let primary = windows.get_window(primary_entity).expect("Failed to get the primary window");
        let icon_buf = Cursor::new(include_bytes!("../../assets/app/icon_256x256.png"));
        if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
            let image = image.into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            let icon = Icon::from_rgba(rgba, width, height).expect("Failed to load window icon");
            primary.set_window_icon(Some(icon));
        }
    }
}
