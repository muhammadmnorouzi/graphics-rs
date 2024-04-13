mod color;
mod math;
mod shapes;
mod traits;

use graphics_rs::{
    graphics::Graphics,
    shapes::{line::Line, triangle::Triangle},
    simple_canvas::SimpleCanvas,
    traits::{canvas::Canvas, canvas_handler::CanvasHandler},
};

// Constants
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

struct MyCanvasHandler {
}

impl<'a> CanvasHandler for MyCanvasHandler {
    fn update<T: Canvas>(&mut self, canvas: &mut T) {
        canvas.change_color(color::BLUE);
        canvas.draw_shape(&mut Triangle::new(0, 0 , 800, 0, 800, 800));
        canvas.change_color(color::GREEN);
        canvas.draw_shape(&mut Triangle::new(0, 0 , 0, 800, 800, 800));
        return;
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
    graphics.run(&mut MyCanvasHandler {
    })?;
    Ok(())
}
