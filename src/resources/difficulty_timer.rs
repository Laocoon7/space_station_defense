use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct DifficultyTimer(pub Timer);

impl Default for DifficultyTimer {
    fn default() -> Self { Self(Timer::from_seconds(10.0, TimerMode::Repeating)) }
}
