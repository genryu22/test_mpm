mod viewer;
use viewer::*;

fn main() {
    println!("Hello, world!");

    let viewer = Viewer::new();
    viewer.open();
}
