use bevy::prelude::*;
use crate::constants;

pub fn create_window() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Pong".to_string(),
            ..default()
        }),
        ..default()
    }
}

pub fn spawn_dotted_line(mut commands: Commands) {
    let dot_color = Color::srgb(1.0, 1.0, 1.0);
    let dot_size = Vec2::new(3., 30.0);
    let gap_size = 10.0;
    let num_dots = (constants::WINDOW_HEIGHT / (dot_size.y + gap_size)) as i32;

    for i in 0..num_dots {
        let y = i as f32 * (dot_size.y + gap_size) - constants::WINDOW_HEIGHT/2.;
        commands.spawn((
            Sprite::from_color(dot_color, dot_size),
            Transform::from_translation(Vec3::new(0., y, 0.)),
        ));
    }
    /* 
    for i in 0..num_dots {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: dot_color,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, i as f32 * (dot_size.y + gap_size) - constants::WINDOW_HEIGHT/2.0, 0.0),
                scale: dot_size,
                ..default()
            },
            ..default()
        });
    }
    */
}