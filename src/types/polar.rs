use bevy::prelude::*;

#[derive(Clone, Copy)]
pub struct Polar {
    pub radius: f32,
    pub theta: f32,
}

impl Polar {
    pub fn new(radius: f32, theta: f32) -> Self {
        Self::default().set_radius(radius).set_theta(theta)
    }

    pub fn set_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn set_theta(mut self, theta: f32) -> Self {
        self.theta = theta;
        self
    }

    pub fn to_vec2(self) -> Vec2 {
        Vec2 {
            x: self.radius * self.theta.cos(),
            y: self.radius * self.theta.sin(),
        }
    }

    pub fn from_vec2(value: Vec2) -> Self {
        let radius = (value.x.powi(2) + value.y.powi(2)).sqrt();
        let theta = value.y.atan2(value.x);
        Self::default().set_radius(radius).set_theta(theta)
    }
}

impl Default for Polar {
    fn default() -> Self {
        Self {
            radius: 1.0,
            theta: 0.0,
        }
    }
}