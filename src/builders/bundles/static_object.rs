use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{components::tags::ObjectTag, types::ScreenLayer};

#[derive(Bundle, Clone)]
pub struct StaticObject {
    pub name: Name,
    pub sprite_bundle: SpriteBundle,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub active_events: ActiveEvents,
    pub sensor: Sensor,
    pub object_tag: ObjectTag,
}

impl StaticObject {
    pub fn new(name: &str, texture: Handle<Image>, color: Color, position: Vec2, collider: Collider) -> Self {
        Self {
            name: Name::new(name.to_string()),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color,
                    ..Default::default()
                },
                texture,
                transform: Transform::from_translation(position.extend(ScreenLayer::Objects.into())),
                visibility: Visibility::Visible,
                ..Default::default()
            },
            collider,
            rigid_body: RigidBody::KinematicPositionBased,
            active_events: ActiveEvents::COLLISION_EVENTS,
            sensor: Sensor,
            object_tag: ObjectTag,
        }
    }
}

impl Default for StaticObject {
    fn default() -> Self {
        Self::new(
            "Object",
            Handle::default(),
            Color::WHITE,
            Vec2::ZERO,
            Collider::default(),
        )
    }
}
