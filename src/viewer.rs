use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::components::*;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn update(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    particle_query: Query<(Entity, &Position), (With<Particle>, Without<Transform>)>,
) {
    for (entity, pos) in particle_query.iter() {
        commands.entity(entity).insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            transform: Transform::from_xyz((pos.0.x - 5.) * 700., (pos.0.y - 5.) * 700., 1.),
            ..default()
        });
    }
}
