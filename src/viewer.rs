use bevy::{prelude::App, DefaultPlugins};

pub fn start_bevy_systems() {
    App::new().add_plugins(DefaultPlugins).run();
}
