use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::components::*;
use crate::constants::*;
use crate::PaddleBundle;

pub fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let window_width = window.resolution.width();
        let padding = 50.;
        let right_paddle_x = window_width/2. - padding;
        let left_paddle_x = -window_width/2. + padding;

        let mesh = Mesh::from(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT));
        let mesh_handle = meshes.add(mesh);

        commands.spawn((
            Player1,
            PaddleBundle::new(right_paddle_x, 0.),
            MaterialMesh2dBundle{
                mesh: mesh_handle.clone().into(),
                material: materials.add(
                    ColorMaterial::from(Color::srgb(0.0, 1.0, 0.0))
                ),
                ..default()
            }
        ));

        commands.spawn((
            Player2,
            PaddleBundle::new(left_paddle_x, 0.),
            MaterialMesh2dBundle{
                mesh: mesh_handle.clone().into(),
                material: materials.add(
                    ColorMaterial::from(Color::srgb(0.0, 0.0, 1.0))
                ),
                ..default()
            }
        ));
    }
}

pub fn move_player1_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle: Query<&mut Velocity, With<Player1>>,
) {
    if let Ok(mut velocity) = paddle.get_single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            velocity.0.y = PADDLE_SPEED;
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            velocity.0.y = -PADDLE_SPEED;
        } else {
            velocity.0.y = 0.0;
        }
    }
}

pub fn move_player2_paddle(
    ball_query: Query<(&Position, &Velocity), With<Ball>>,
    mut paddle_query: Query<(&mut Position, &mut Velocity), (With<Player2>, Without<Ball>)>,
) {
    if let Ok((ball_position, ball_velocity)) = ball_query.get_single() {
        if let Ok((paddle_position, mut velocity)) = paddle_query.get_single_mut() {
            let paddle_center_y = paddle_position.0.y;
            let ball_y = ball_position.0.y;
            let ball_vel = ball_velocity.0.x;
            if ball_vel < 0. {
                if (ball_y - paddle_center_y).abs() < PADDLE_HEIGHT / 2.0 {
                    velocity.0.y = 0.;
                } else if ball_y > paddle_center_y {
                    velocity.0.y = PADDLE_SPEED;
                } else {
                    velocity.0.y = -PADDLE_SPEED;
                }
            } else {
                if (0. - paddle_center_y).abs() < PADDLE_HEIGHT / 2.0 {
                    velocity.0.y = 0.;
                } else if ball_y > paddle_center_y {
                    velocity.0.y = PADDLE_SPEED;
                } else {
                    velocity.0.y = -PADDLE_SPEED;
                }
            }
        }
    }
}

pub fn move_paddles(
    mut paddle: Query<(&mut Position, &Velocity), With<Paddle>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let window_height = window.resolution.height();

        for (mut position, velocity) in &mut paddle {
            let new_position = position.0 + velocity.0 * PADDLE_SPEED;
            if new_position.y.abs() < window_height / 2.0 - PADDLE_HEIGHT / 2.0 {
                position.0 = new_position;
            }
        }
    }
}