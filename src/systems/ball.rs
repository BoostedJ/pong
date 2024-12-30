use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::components::*;
use crate::constants::*;
use crate::BallBundle;

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