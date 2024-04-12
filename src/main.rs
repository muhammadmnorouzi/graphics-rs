mod color;
mod traits;
mod shapes;

use graphics_rs::{
    graphics::{Graphics, GraphicsHandler}, shapes::circle::Circle, simple_canvas::SimpleCanvas, traits::canvas::Canvas
};

struct CustomeHandler {
    circle: Circle,
}

impl GraphicsHandler for CustomeHandler {
    fn draw(&mut self, canvas: &mut SimpleCanvas) {
        canvas.draw_shape(&mut self.circle);
    }
}

// Constants
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn main() -> Result<(), String> {
    let canvas = SimpleCanvas::new(WIDTH as usize, HEIGHT as usize, None, false, None);
    let mut graphics = Graphics::create(canvas);
    graphics.run(&mut CustomeHandler {
        circle: Circle::new(400, 400, 200, color::GREEN),
    })?;
    Ok(())
}
