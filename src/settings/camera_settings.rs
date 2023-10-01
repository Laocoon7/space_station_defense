use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CameraSettings {
    pub tile_size: u32,
}

impl Default for CameraSettings {
    fn default() -> Self { Self { tile_size: 1 } }
}
