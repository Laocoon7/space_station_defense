use std::path::PathBuf;

use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
};
use serde::Deserialize;

use crate::objects_plugin::loaders::builders::SpaceStationBuilder;

#[derive(TypeUuid, TypePath, Clone)]
#[uuid = "fb5ab52f-5d6b-4e2d-9403-19ecf577e852"]
pub struct SpaceStationDefinition {
    pub name: String,
    pub idle_animation: Handle<fortress::prelude::AnimationDefinition>,
}

impl SpaceStationDefinition {
    pub fn to_builder(&self) -> SpaceStationBuilder {
        SpaceStationBuilder {
            
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct SpaceStationRecord {
    #[serde(alias = "type")]
    pub name: String,
    pub idle_animation: PathBuf,
}

impl SpaceStationRecord {
    pub fn to_definition(&self, idle_animation: &Handle<AnimationDefinition>) -> SpaceStationDefinition {
        SpaceStationDefinition {
            name: self.name.clone(),
            idle_animation: idle_animation.clone(),
        }
    }
}
