use bevy::prelude::*;

use super::{DynamicObject, StaticObject};
use crate::components::{tags::EnemyTag, Health};

#[derive(Bundle, Clone)]
pub struct EnemyBundle {
    pub dynamic_object: DynamicObject,
    pub health: Health,
    pub enemy_tag: EnemyTag,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            dynamic_object: DynamicObject {
                static_object: StaticObject {
                    name: Name::new("Enemy"),
                    ..Default::default()
                },
                ..Default::default()
            },
            health: Health::default(),
            enemy_tag: EnemyTag,
        }
    }
}
