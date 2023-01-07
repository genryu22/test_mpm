use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::components::*;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn update(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    init_particle_query: Query<(Entity, &Position), (With<Particle>, Without<Transform>)>,
    mut particle_query: Query<(&Position, &mut Transform), (With<Particle>, With<Transform>)>,
) {
    for (entity, pos) in init_particle_query.iter() {
        commands.entity(entity).insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            transform: Transform::from_xyz((pos.0.x - 32.) * 10., (pos.0.y - 32.) * 10., 1.),
            ..default()
        });
    }

    for (pos, mut transform) in particle_query.iter_mut() {
        *transform = Transform::from_xyz((pos.0.x - 32.) * 10., (pos.0.y - 32.) * 10., 1.);
    }
}
