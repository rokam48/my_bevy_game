use bevy::prelude::*;
use hello::HelloPlugin;

pub mod hello;

pub fn run_game() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}
