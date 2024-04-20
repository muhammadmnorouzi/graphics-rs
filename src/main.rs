#![allow(dead_code)]

mod color;
mod math;
mod shapes;
mod tools;
mod traits;

use graphics_rs::{graphics::Graphics, simple_canvas::SimpleCanvas};

// Constants
const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn main() -> Result<(), String> {
    let antialiasing = false;
    let antialiasing_resolution = 1;
    let fill_color = Some(color::BLACK);

    let mut canvas = SimpleCanvas::new(
        WIDTH as usize,
        HEIGHT as usize,
        fill_color,
        antialiasing,
        antialiasing_resolution,
    );

    let mut graphics = Graphics::create(&mut canvas)?;
    // graphics.run()?;

    Ok(())
}
