use bevy::prelude::*;

mod components;
mod simulater;
mod viewer;

fn main() {
    println!("Hello, world!");

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(components::Settings {
            dt: 1e-2,
            gravity: -1e-2,
            dynamic_viscosity: 1e-2,
            space_width: 10.,
            grid_width: 200,
            c: 0.,
            eos_power: 0.,
        })
        .add_startup_system(simulater::setup)
        .add_startup_system(viewer::setup)
        .add_system(viewer::update)
        .add_system_set(simulater::create_system_set())
        .run();
}
