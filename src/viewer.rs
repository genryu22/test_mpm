use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::components::*;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn update(
    mut commands: Commands,
    settings: Res<Settings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    init_particle_query: Query<(Entity, &Position), (With<Particle>, Without<Transform>)>,
    mut particle_query: Query<
        (
            &Position,
            &mut Transform,
            &ColorFactor,
            &mut Handle<ColorMaterial>,
        ),
        (With<Particle>, With<Transform>),
    >,
) {
    for (entity, pos) in init_particle_query.iter() {
        commands.entity(entity).insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            transform: Transform::from_xyz(
                (pos.0.x - settings.space_width / 2.) * 10.,
                (pos.0.y - settings.space_width / 2.) * 10.,
                1.,
            ),
            ..default()
        });
    }

    for (pos, mut transform, cf, mut colorMaterial) in particle_query.iter_mut() {
        *transform = Transform::from_xyz(
            (pos.0.x - settings.space_width / 2.) * 10.,
            (pos.0.y - settings.space_width / 2.) * 10.,
            1.,
        );
        let mut color_mat = materials.get_mut(&colorMaterial).unwrap();
        let f = 1. / (1. + (1e-1 * (cf.0 as f64 - 30.)).exp());
        color_mat.color = Color::rgb(f as f32 * 0.8, f as f32 * 0.8, 1.);
    }
}
