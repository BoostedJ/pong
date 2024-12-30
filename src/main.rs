use bevy::prelude::*;
mod systems;
mod constants;

use systems::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(create_window()))
    .add_systems(Startup, (spawn_dotted_line, spawn_camera))
    .run();
}
