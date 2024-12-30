use bevy::prelude::*;
mod systems;
mod constants;
mod components;
mod bundles;

use systems::*;
use components::*;
use bundles::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(create_window()))
    .add_systems(Startup, (spawn_dotted_line, spawn_ball, spawn_paddles, spawn_camera))
    .run();
}
