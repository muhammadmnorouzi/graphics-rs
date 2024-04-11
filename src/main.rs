mod canvas;
mod color;

use graphics_rs::{
    canvas::Canvas,
    graphics::{Graphics, GraphicsHandler},
};

struct CustomeHandler {
    radius: usize,
}

impl GraphicsHandler for CustomeHandler {
    fn draw(&mut self, canvas: &mut graphics_rs::canvas::Canvas) {
        canvas.change_color(color::BLACK);
        canvas.fill();
        canvas.change_color(color::RED);
        canvas.fill_circle(400, 400, self.radius);

        self.radius += 1;
        self.radius = self.radius.clamp(0, canvas.width())
    }
}

// Constants
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn main() -> Result<(), String> {
    let canvas = Canvas::create(WIDTH as usize, HEIGHT as usize, None);
    let mut graphics = Graphics::create(canvas);
    graphics.run(&mut CustomeHandler { radius: 0 })?;
    Ok(())
}
