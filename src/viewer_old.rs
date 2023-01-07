use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use piston_window::*;

pub struct Viewer {
    points: Arc<Mutex<Vec<Point>>>,
    prev_points: Vec<Point>,
}

#[derive(Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Viewer {
    pub fn new() -> Self {
        Viewer {
            points: Arc::new(Mutex::new(vec![])),
            prev_points: vec![],
        }
    }

    pub fn open(mut self) {
        let (width, height) = (640, 480);
        let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [width, height])
            .exit_on_esc(true)
            .build()
            .unwrap();
        let radius = 5.;
        let circle_rect = [-radius, -radius, radius, radius];
        let color = [1.0, 0.0, 0.0, 1.0];
        while let Some(event) = window.next() {
            if let Some(Button::Keyboard(key)) = event.press_args() {
                if key == Key::Space {}
            }

            let points_to_draw = match self.points.try_lock() {
                Ok(points) => points.to_vec(),
                _ => self.prev_points.to_vec(),
            };

            window.draw_2d(&event, |context, graphics, _device| {
                clear([1.0; 4], graphics);
                for p in points_to_draw.iter() {
                    let v_p = convert_for_viewport((width, height), 10., 5., (p.x, p.y));
                    let transform = context.transform.trans(v_p.0, v_p.1);
                    rectangle(color, circle_rect, transform, graphics);
                }
            });

            self.prev_points = points_to_draw;
        }
    }
}

fn convert_for_viewport(
    size: (u32, u32),
    space_size: f64,
    offset: f64,
    pos: (f64, f64),
) -> (f64, f64) {
    let (cx, cy) = (size.0 as f64 / 2., size.1 as f64 / 2.);
    let ratio = size.1 as f64 / space_size;

    (
        (pos.0 - offset) * ratio + cx,
        (pos.1 - offset) * ratio * -1. + cy,
    )
}
