#![allow(dead_code)]

mod color;
mod math;
mod shapes;
mod tools;
mod traits;

use graphics_rs::{
    graphics::Graphics,
    shapes::cube3d::Cube3D,
    simple_canvas::SimpleCanvas,
    traits::{canvas::Canvas, canvas_handler::CanvasHandler},
};

// Constants
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

struct MyCanvasHandler {}

impl<'a> CanvasHandler for MyCanvasHandler {
    fn update<T: Canvas>(&mut self, canvas: &mut T) {
        canvas.change_color(color::BLACK);
        canvas.fill();
        canvas.change_color(color::GREEN);
        canvas.draw_shape(&mut Cube3D::new(10));
    }
}

fn main() -> Result<(), String> {
    let mut canvas =
        SimpleCanvas::new(WIDTH as usize, HEIGHT as usize, Some(color::BLACK), true, 4);

    let mut graphics = Graphics::create(&mut canvas)?;
    graphics.run(&mut MyCanvasHandler {})?;
    Ok(())
}
