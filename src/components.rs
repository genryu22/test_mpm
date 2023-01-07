use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Velocity(Vec2);

#[derive(Component, Debug)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct Mass(pub f32);

#[derive(Component)]
pub struct C(Mat2);

#[derive(Component, Default)]
pub struct Particle {}

#[derive(Component)]
pub struct Grid {
    pub nodes: Vec<Node>,
}

pub struct Node {
    v: Vec2,
    v_star: Vec2,
    force: Vec2,
    mass: f32,
}

impl Node {
    pub fn new() -> Node {
        Node {
            v: Vec2::ZERO,
            v_star: Vec2::ZERO,
            force: Vec2::ZERO,
            mass: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.v = Vec2::ZERO;
        self.v_star = Vec2::ZERO;
        self.force = Vec2::ZERO;
        self.mass = 0.0;
    }
}

#[derive(Resource, Debug)]
pub struct Settings {
    pub dt: f64,
    pub gravity: f64,
    pub dynamic_viscosity: f64,
    pub space_width: f64,
    pub grid_width: usize,

    pub c: f64,
    pub eos_power: f64,
}

impl Settings {
    pub fn cell_width(&self) -> f64 {
        self.space_width / (self.grid_width as f64)
    }
}
