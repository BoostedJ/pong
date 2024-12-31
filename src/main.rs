#![windows_subsystem = "windows"]
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
    .init_resource::<Score>()
    .add_event::<Scored>()
    .add_systems(Startup, (spawn_dotted_line, spawn_ball, spawn_paddles, spawn_camera, spawn_scoreboard, spawn_boundary))
    .add_systems(Update, (
        move_ball,
        move_player1_paddle,
        move_player2_paddle,
        detect_scoring,
        respawn_ball.after(detect_scoring),
        update_score.after(detect_scoring),
        update_scoreboard.after(detect_scoring),
        update_entity_positions.after(move_ball),
        move_paddles.after(move_player1_paddle),
        handle_collisions.after(move_ball),
    ))
    .run();
}
