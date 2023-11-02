use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Reflect, Component, Clone, Copy)]
#[reflect(Serialize, Deserialize)]
pub struct CameraTag;
