use bevy::prelude::*;

use crate::components::Mover;

pub fn move_movers(time: Res<Time>, mut q_movers: Query<(&mut Transform, &Mover)>) {
    for (mut transform, mover) in q_movers.iter_mut() {
        mover.apply(&mut transform, &time);
    }
}
