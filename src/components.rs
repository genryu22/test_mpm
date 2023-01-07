use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component, Debug)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct Mass(pub f32);

#[derive(Component, Default)]
pub struct C(pub Mat2);

#[derive(Component, Default)]
pub struct Particle {}

#[derive(Component)]
pub struct Grid {
    pub nodes: Vec<Vec<Node>>,
    pub grid_width: usize,
}

pub struct GridMutIterator<'a> {
    x_index: usize,
    y_index: usize,
    grid: &'a mut Grid,
}

impl<'a> Iterator for GridMutIterator<'a> {
    type Item = &'a mut Node;

    fn next(&mut self) -> Option<Self::Item> {
        let current_x = self.x_index;
        let current_y = self.y_index;
        if self.x_index >= self.grid.grid_width + 1 {
            None
        } else {
            self.y_index += 1;
            if self.y_index >= self.grid.grid_width + 1 {
                self.y_index = 0;
                self.x_index += 1;
            }
            unsafe {
                Some(
                    self.grid
                        .nodes
                        .as_mut_ptr()
                        .add(current_x)
                        .as_mut()?
                        .as_mut_ptr()
                        .add(current_y)
                        .as_mut()?,
                )
            }
        }
    }
}

impl Grid {
    pub fn iter_mut(&mut self) -> GridMutIterator {
        GridMutIterator {
            x_index: 0,
            y_index: 0,
            grid: self,
        }
    }

    pub fn get_mut(&mut self, index_x: i32, index_y: i32) -> Option<&mut Node> {
        match self.inside_grid(index_x, index_y) {
            true => self
                .nodes
                .get_mut(index_x as usize)?
                .get_mut(index_y as usize),
            false => None,
        }
    }

    pub fn get(&self, index_x: i32, index_y: i32) -> Option<&Node> {
        match self.inside_grid(index_x, index_y) {
            true => self.nodes.get(index_x as usize)?.get(index_y as usize),
            false => None,
        }
    }

    fn inside_grid(&self, index_x: i32, index_y: i32) -> bool {
        0 <= index_x
            && index_x < self.grid_width as i32 + 1
            && 0 <= index_y
            && index_y < self.grid_width as i32 + 1
    }
}

pub struct Node {
    pub v: Vec2,
    pub v_star: Vec2,
    pub force: Vec2,
    pub mass: f32,

    pub index_x: usize,
    pub index_y: usize,
}

impl Node {
    pub fn new(index_x: usize, index_y: usize) -> Node {
        Node {
            v: Vec2::ZERO,
            v_star: Vec2::ZERO,
            force: Vec2::ZERO,
            mass: 0.0,
            index_x,
            index_y,
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
    pub dt: f32,
    pub gravity: f32,
    pub dynamic_viscosity: f32,
    pub space_width: f32,
    pub grid_width: usize,

    pub rho_0: f32,

    pub rest_density: f32,
    pub eos_stiffness: f32,
    pub eos_power: f32,

    pub e: f32,
    pub nu: f32,
}

impl Settings {
    pub fn cell_width(&self) -> f32 {
        self.space_width / (self.grid_width as f32)
    }

    pub fn particle_volume(&self) -> f32 {
        let dx = self.cell_width();
        (dx / 2.) * (dx / 2.)
    }

    pub fn lambda(&self) -> f32 {
        self.e * self.nu / ((1. + self.nu) * (1. - 2. * self.nu))
    }
}
