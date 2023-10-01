use bevy::prelude::*;

pub struct Polar {
    pub r: f32,
    pub t: f32,
}

impl From<Vec2> for Polar {
    fn from(value: Vec2) -> Self {
        Self {
            r: (value.x.powi(2) + value.y.powi(2)).sqrt(),
            t: (value.y).atan2(value.x),
        }
    }
}

impl From<Polar> for Vec2 {
    fn from(value: Polar) -> Self {
        Self {
            x: value.r * value.t.cos(),
            y: value.r * value.t.sin(),
        }
    }
}
