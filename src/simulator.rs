use crate::components::*;
use bevy::prelude::*;

pub fn setup(mut commands: Commands, settings: Res<Settings>) {
    println!("{:?}", settings);

    let grid_width = settings.grid_width;
    let cell_size = settings.cell_width() as f32;

    let p_dist = cell_size / 2. as f32;

    let pos_x_min = 16 as f32;
    let pos_x_max = 48 as f32;
    let num_x = ((pos_x_max - pos_x_min) / p_dist + 0.5) as usize;

    for i_y in 0..num_x {
        for i_x in 0..num_x {
            let p = Vec2::new(
                (pos_x_max - pos_x_min) * (i_x as f32 + 0.5) / num_x as f32 + pos_x_min,
                (pos_x_max - pos_x_min) * (i_y as f32 + 0.5) / num_x as f32 + pos_x_min,
            );
            let mass = (settings.rho_0 * (pos_x_max - pos_x_min) * (pos_x_max - pos_x_min))
                / (num_x * num_x) as f32;
            commands.spawn((
                Particle::default(),
                Mass(mass),
                Velocity::default(),
                Position(p),
                C::default(),
            ));
        }
    }

    let mut grid: Vec<Vec<crate::components::Node>> = Vec::with_capacity(grid_width + 1);
    for _x in 0..(grid_width + 1) {
        let mut column: Vec<crate::components::Node> = Vec::with_capacity(grid_width + 1);
        for _y in 0..(grid_width + 1) {
            column.push(crate::components::Node::new(_x, _y));
        }
        grid.push(column);
    }

    commands.spawn(Grid {
        nodes: grid,
        grid_width,
    });
}

pub fn create_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(clear_grid.label("clear_grid"))
        .with_system(distribute_mass.label("distribute_mass").after("clear_grid"))
        .with_system(p2g.label("p2g").after("distribute_mass"))
        .with_system(update_grid.label("update_grid").after("p2g"))
        .with_system(g2p.label("g2p").after("update_grid"))
        .with_system(mouse_button_input.after("g2p"))
}

fn clear_grid(mut grid_query: Query<&mut Grid>) {
    let mut grid = grid_query.single_mut();
    for n in grid.iter_mut() {
        n.reset();
    }
}

fn distribute_mass(
    settings: Res<Settings>,
    particle_query: Query<(&Position, &Mass, &Velocity, &C), With<Particle>>,
    mut grid_query: Query<&mut Grid>,
) {
    let mut grid = grid_query.single_mut();
    for (p, m, v, c) in particle_query.iter() {
        let base = (p.0 * settings.cell_width() - 0.5).floor();
        let fx = p.0 * settings.cell_width() - base;

        let w = vec![
            0.5 * (1.5 - fx).powf(2.),
            0.75 - (fx - 1.).powf(2.),
            0.5 * (fx - 0.5).powf(2.),
        ];

        for gx in 0..3 {
            for gy in 0..3 {
                let weight = w[gx].x * w[gy].y;
                let dpos = (Vec2::new(gx as f32, gy as f32) - fx) * settings.cell_width();

                let q = c.0 * dpos;
                let mass_contrib = weight * m.0;

                let node = grid.get_mut(base.x as i32 + gx as i32, base.y as i32 + gy as i32);
                if let Some(node) = node {
                    node.mass += mass_contrib;
                    node.v += mass_contrib * (v.0 + q);
                }
            }
        }
    }
}

fn p2g(
    settings: Res<Settings>,
    particle_query: Query<(&Position, &Mass, &Velocity, &C), With<Particle>>,
    mut grid_query: Query<&mut Grid>,
) {
    let mut grid = grid_query.single_mut();
    for (p, m, v, c) in particle_query.iter() {
        let base = (p.0 * settings.cell_width() - 0.5).floor();
        let fx = p.0 * settings.cell_width() - base;

        let w = vec![
            0.5 * (1.5 - fx).powf(2.),
            0.75 - (fx - 1.).powf(2.),
            0.5 * (fx - 0.5).powf(2.),
        ];

        let eq_16_term_0 = {
            let mut density = 0.;
            for gx in 0..3 {
                for gy in 0..3 {
                    let weight = w[gx].x * w[gy].y;
                    let node = grid.get(base.x as i32 + gx as i32, base.y as i32 + gy as i32);
                    if let Some(node) = node {
                        let dx = settings.cell_width();
                        density += weight * node.mass / (dx * dx);
                    }
                }
            }
            let density = density;
            let volume = m.0 / density;

            let mut pressure = settings.eos_stiffness
                * ((density / settings.rest_density).powf(settings.eos_power) - 1.);
            if pressure < -1e-1 {
                pressure = -1e-1;
            }
            let pressure = pressure;

            let mut stress = Mat2::from_diagonal(Vec2::new(-pressure, -pressure));

            let mut strain = c.0;

            let anti_trace = strain.col(1).x + strain.col(0).y;
            strain.col_mut(1).x = anti_trace;
            strain.col_mut(0).y = anti_trace;

            stress += settings.dynamic_viscosity * strain;

            -volume * 4. * stress * settings.dt
        };

        for gx in 0..3 {
            for gy in 0..3 {
                let weight = w[gx].x * w[gy].y;
                let dpos = (Vec2::new(gx as f32, gy as f32) - fx) * settings.cell_width();

                let node = grid.get_mut(base.x as i32 + gx as i32, base.y as i32 + gy as i32);
                if let Some(node) = node {
                    let momentum = weight * eq_16_term_0 * dpos;
                    node.v += momentum;
                }
            }
        }
    }
}

fn update_grid(settings: Res<Settings>, mut grid_query: Query<&mut Grid>) {
    let mut grid = grid_query.single_mut();

    for node in grid.iter_mut() {
        if node.mass <= 0. {
            continue;
        }

        node.v /= node.mass;
        node.v += settings.dt * Vec2::new(0., settings.gravity);

        if node.index_x < 2 || node.index_x > settings.grid_width - 2 {
            node.v.x = 0.;
        }
        if node.index_y < 2 || node.index_y > settings.grid_width - 2 {
            node.v.y = 0.;
        }
    }
}

fn g2p(
    settings: Res<Settings>,
    mut particle_query: Query<(&mut Position, &Mass, &mut Velocity, &mut C), With<Particle>>,
    grid_query: Query<&Grid>,
) {
    let grid = grid_query.single();

    particle_query.par_for_each_mut(10, |(mut p, m, mut v, mut c)| {
        v.0 = Vec2::ZERO;

        let base = (p.0 * settings.cell_width() - 0.5).floor();
        let fx = p.0 * settings.cell_width() - base;

        let w = vec![
            0.5 * (1.5 - fx).powf(2.),
            0.75 - (fx - 1.).powf(2.),
            0.5 * (fx - 0.5).powf(2.),
        ];

        let mut b = Mat2::ZERO;
        for gx in 0..3 {
            for gy in 0..3 {
                let weight = w[gx].x * w[gy].y;
                let dpos = (Vec2::new(gx as f32, gy as f32) - fx) * settings.cell_width();

                let node = grid.get(base.x as i32 + gx as i32, base.y as i32 + gy as i32);
                if let Some(node) = node {
                    let weighted_velocity = node.v * weight;
                    b += Mat2::from_cols(weighted_velocity * dpos.x, weighted_velocity * dpos.y);
                    v.0 += weighted_velocity;
                }
            }
        }

        c.0 = b * (4. / (settings.cell_width() * settings.cell_width()));

        p.0 += v.0 * settings.dt;

        p.0 = p.0.clamp(
            Vec2::new(settings.cell_width(), settings.cell_width()),
            Vec2::new(
                settings.space_width as f32 - settings.cell_width(),
                settings.space_width as f32 - settings.cell_width(),
            ),
        );

        let x_n = p.0 + v.0;
        let wall_min = 3. * settings.cell_width();
        let wall_max = settings.space_width - 3. * settings.cell_width();
        if x_n.x < wall_min {
            v.0.x += wall_min - x_n.x;
        }
        if x_n.x > wall_max {
            v.0.x += wall_max - x_n.x;
        }
        if x_n.y < wall_min {
            v.0.y += wall_min - x_n.y;
        }
        if x_n.y > wall_max {
            v.0.y += wall_max - x_n.y;
        }
    });
}

fn mouse_button_input(
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    settings: Res<Settings>,
    mut particle_query: Query<(&Position, &mut Velocity), With<Particle>>,
) {
    if buttons.pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        if let Some(cursor_pos) = window.cursor_position() {
            let s_pos = (cursor_pos - Vec2::new(window.width() / 2., window.height() / 2.)) / 10.
                + Vec2::new(settings.space_width / 2., settings.space_width / 2.);
            for (p, mut v) in particle_query.iter_mut() {
                let dist = p.0.distance(s_pos);
                v.0 += settings.dt
                    * 1e-2
                    * (p.0 - s_pos).normalize()
                    * (settings.space_width * settings.space_width)
                    / (dist * dist);
            }
        }
    }
}
