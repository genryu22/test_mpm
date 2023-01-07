use bevy::prelude::*;

mod components;
mod simulator;
mod viewer;

fn main() {
    println!("Hello, world!");

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(components::Settings {
            dt: 2e-1,
            gravity: -3e-1,
            dynamic_viscosity: 1e-1,
            space_width: 64.,
            grid_width: 64,

            rho_0: 4.,

            rest_density: 4.,
            eos_stiffness: 10.,
            eos_power: 4.,

            e: 5e3,
            nu: 0.2,
        })
        .add_startup_system(simulator::setup)
        .add_startup_system(viewer::setup)
        .add_system(viewer::update)
        .add_system_set(simulator::create_system_set())
        .run();
}
