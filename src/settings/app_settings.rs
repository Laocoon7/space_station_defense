use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use bevy::prelude::*;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use super::{CameraSettings, WindowSettings};

const PATH: &'static str = "settings.toml";

lazy_static! {
    static ref APP_SETTINGS: RwLock<AppSettings> = RwLock::new(AppSettings::load());
}

#[derive(Serialize, Deserialize)]
pub struct AppSettings {
    pub camera_settings: CameraSettings,
    pub window_settings: WindowSettings,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            camera_settings: CameraSettings::default(),
            window_settings: WindowSettings::default(),
        }
    }
}

impl AppSettings {
    pub fn camera_settings(&self) -> &CameraSettings { &self.camera_settings }

    pub fn camera_settings_mut(&mut self) -> &mut CameraSettings { &mut self.camera_settings }

    pub fn window_settings(&self) -> &WindowSettings { &self.window_settings }

    pub fn window_settings_mut(&mut self) -> &mut WindowSettings { &mut self.window_settings }

    fn load() -> Self {
        match std::fs::read_to_string(PATH) {
            Ok(contents) => match toml::from_str(&contents) {
                Ok(data) => data,
                Err(e) => {
                    error!("{e}");
                    Self::default()
                },
            },
            Err(e) => {
                error!("{e}");
                Self::default()
            },
        }
    }

    pub fn save(&self) {
        match toml::to_string_pretty(self) {
            Ok(contents) => match std::fs::write(PATH, contents) {
                Err(e) => error!("{e}"),
                _ => (),
            },
            Err(e) => error!("{e}"),
        }
    }

    pub fn lock() -> RwLockReadGuard<'static, AppSettings> {
        APP_SETTINGS.read().expect("Failed to get app settings read lock")
    }

    pub fn lock_mut() -> RwLockWriteGuard<'static, AppSettings> {
        APP_SETTINGS.write().expect("Failed to get app settings write lock")
    }
}
