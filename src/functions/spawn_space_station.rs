use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    components::{tags::SpaceStationTag, Health},
    resources::SpaceStationImage,
    types::ScreenLayer,
};

pub fn spawn_space_station(commands: &mut Commands, space_station_image: &Res<SpaceStationImage>) {
    commands.spawn((
        Name::new("Space Station"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, ScreenLayer::Objects.into()),
            texture: space_station_image.0.clone(),
            visibility: Visibility::Visible,
            ..Default::default()
        },
        Health {
            current: 10,
            max: 10,
        },
        Collider::ball(64.0),
        Sleeping::disabled(),
        // CollisionGroups::new(Group::GROUP_1, Group::GROUP_1 | Group::GROUP_2),
        RigidBody::Dynamic,
        GravityScale(0.0),
        ActiveEvents::COLLISION_EVENTS,
        // Sensor,
        SpaceStationTag,
    ));
}
