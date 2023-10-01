use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Default for Health {
    fn default() -> Self {
        Health {
            current: 10,
            max: 10,
        }
    }
}
