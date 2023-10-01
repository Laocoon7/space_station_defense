use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WindowSettings {
    pub resolution: UVec2,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            resolution: UVec2 { x: 1920, y: 1080 },
        }
    }
}
