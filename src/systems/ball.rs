use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::components::*;
use crate::constants::*;
use crate::BallBundle;

// Creates a new ball entity with a mesh and material
pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh::from(Circle::new(BALL_SIZE));
    let color = ColorMaterial::from(Color::srgb(1.0, 0.0, 0.0));

    let mesh_handle = meshes.add(shape);
    let material_handle = materials.add(color);

    commands.spawn((
        BallBundle::new(1.0, 0.0),
        MaterialMesh2dBundle {
            mesh:mesh_handle.into(),
            material:bevy::prelude::MeshMaterial2d(material_handle),
            ..default()
        }
    ));
}

// Takes single parameter, ball, (mutable query of tuples), only including entities that have "Ball" component
// Matches to retreieve a single mutable reference to position and velocity returning Result type which is either Ok, or error
// If successful, function will update the position of the ball
pub fn move_ball(
    mut ball:Query<(&mut Position, &Velocity), With<Ball>>
) {
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0 += velocity.0 * BALL_SPEED;
    }
}

// Takes single parameter, any entity with a Transform and Position component
// Updates translation field representing the position of the entity
pub fn update_entity_positions(
    mut ball: Query<(&mut Transform, &Position)>
) {
    for (mut transform, position) in ball.iter_mut() {
        transform.translation = position.0.extend(0.);
    }
}