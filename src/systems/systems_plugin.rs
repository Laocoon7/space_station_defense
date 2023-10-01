use bevy::prelude::*;

use super::{handle_collisions, move_movers, spawn_camera, spawn_enemies, start_game, up_difficulty};
use crate::types::{AppState, GameState};

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, skip_main_menu));

        app.add_systems(Update, start_game.run_if(in_state(GameState::GenerateGame)));
        app.add_systems(
            Update,
            (spawn_enemies, move_movers).chain().run_if(in_state(GameState::Playing)),
        );
        app.add_systems(
            PostUpdate,
            (up_difficulty, handle_collisions).run_if(in_state(GameState::Playing)),
        );
    }
}

fn skip_main_menu(mut app_state: ResMut<NextState<AppState>>) { app_state.set(AppState::Game); }
