mod color;
mod math;
mod shapes;
mod traits;

use graphics_rs::{
    graphics::Graphics,
    shapes::{circle::Circle, line::Line},
    simple_canvas::SimpleCanvas,
    traits::{canvas::Canvas, canvas_handler::CanvasHandler},
};

// Constants
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

struct MyCanvasHandler;

impl CanvasHandler for MyCanvasHandler {
    fn update<T: Canvas>(&mut self, canvas: &mut T) {
        canvas.draw_shape(&mut Line::new(100, 100, 500 , 900, color::RED));
    }
}

fn main() -> Result<(), String> {
    let mut canvas = SimpleCanvas::new(
        WIDTH as usize,
        HEIGHT as usize,
        Some(color::BLACK),
        false,
        None,
    );
    let mut graphics = Graphics::create(&mut canvas)?;
    graphics.run(&mut MyCanvasHandler {})?;
    Ok(())
}
