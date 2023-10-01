use bevy::prelude::*;
use types::{AppState, GameState};

use crate::{resources::ResourcesPlugin, systems::SystemsPlugin, windows::WindowsPlugin};

pub mod builders;
pub mod components;
pub mod functions;
pub mod resources;
pub mod settings;
pub mod types;

mod systems;
mod windows;

fn main() {
    let mut app = App::new();

    app.add_state::<AppState>();
    app.add_state::<GameState>();

    app.add_plugins((WindowsPlugin, ResourcesPlugin, SystemsPlugin));

    app.run();
}
