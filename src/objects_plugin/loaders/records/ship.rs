use std::path::PathBuf;

use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
};
use serde::Deserialize;

use crate::objects_plugin::loaders::builders::ShipBuilder;

#[derive(TypeUuid, TypePath, Clone)]
#[uuid = "9f30ccb7-0628-4635-ab14-e2eeaa64dd39"]
pub struct ShipDefinition {
    pub name: String,
    pub idle_animation: Handle<fortress::prelude::AnimationDefinition>,
    pub mass: f32,
    pub speed: f32,
    pub drag: f32,
}

impl ShipDefinition {
    pub fn to_builder(&self) -> ShipBuilder {
        ShipBuilder {
            
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct ShipRecord {
    #[serde(alias = "type")]
    pub name: String,
    pub idle_animation: PathBuf,
    pub mass: f32,
    pub speed: f32,
    pub drag: f32,
}

impl ShipRecord {
    pub fn to_definition(&self, idle_animation: &Handle<AnimationDefinition>) -> ShipDefinition {
        ShipDefinition {
            name: self.name.clone(),
            idle_animation: idle_animation.clone(),
            mass: self.mass,
            speed: self.speed,
            drag: self.drag,
        }
    }
}
