use crate::components::*;
use bevy::prelude::*;

pub fn setup(mut commands: Commands, settings: Res<Settings>) {
    println!("{:?}", settings);

    let grid_width = settings.grid_width;
    let cell_size = settings.cell_width() as f32;

    let p_dist = cell_size / 2. as f32;

    let pos_x_min = 4.5 as f32;
    let pos_x_max = 5.5 as f32;
    let num_x = ((pos_x_max - pos_x_min) / p_dist + 0.5) as usize;

    for i_y in 0..num_x {
        for i_x in 0..num_x {
            let p = Vec2::new(
                (pos_x_max - pos_x_min) * (i_x as f32 + 0.5) / num_x as f32 + pos_x_min,
                (pos_x_max - pos_x_min) * (i_y as f32 + 0.5) / num_x as f32 + pos_x_min,
            );
            let mass =
                (1. * (pos_x_max - pos_x_min) * (pos_x_max - pos_x_min)) / (num_x * num_x) as f32;
            commands.spawn((
                Particle::default(),
                Mass(mass),
                Velocity::default(),
                Position(p),
            ));
        }
    }

    let mut grid: Vec<crate::components::Node> =
        Vec::with_capacity((grid_width + 1) * (grid_width + 1));
    for _i in 0..(grid_width + 1) * (grid_width + 1) {
        grid.push(crate::components::Node::new());
    }

    commands.spawn(Grid { nodes: grid });
}

pub fn create_system_set() -> SystemSet {
    SystemSet::new()
}

fn clear_grid() {}
