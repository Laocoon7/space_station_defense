use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::StaticObject;
use crate::components::{MovementType, Mover};

#[derive(Bundle, Clone)]
pub struct DynamicObject {
    pub static_object: StaticObject,
    pub mover: Mover,
}

impl DynamicObject {
    pub fn new(
        name: &str,
        texture: Handle<Image>,
        color: Color,
        position: Vec2,
        collider: Collider,
        movement_type: MovementType,
    ) -> Self {
        Self {
            static_object: StaticObject {
                rigid_body: RigidBody::Dynamic,
                ..StaticObject::new(name, texture, color, position, collider)
            },
            mover: Mover { movement_type },
        }
    }
}

impl Default for DynamicObject {
    fn default() -> Self {
        Self {
            static_object: StaticObject {
                rigid_body: RigidBody::Dynamic,
                ..Default::default()
            },
            mover: Mover::default(),
        }
    }
}
