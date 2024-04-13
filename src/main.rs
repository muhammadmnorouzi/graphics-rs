mod color;
mod math;
mod shapes;
mod traits;

use graphics_rs::{
    graphics::Graphics,
    shapes::line::Line,
    simple_canvas::SimpleCanvas,
    traits::{canvas::Canvas, canvas_handler::CanvasHandler},
};

// Constants
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

struct MyCanvasHandler {
    step: i32,
    x: usize,
    y: usize,
}

impl<'a> CanvasHandler for MyCanvasHandler {
    fn update<T: Canvas>(&mut self, canvas: &mut T) {
        canvas.change_color(color::BLACK);
        canvas.fill();
        canvas.change_color(color::RED);
        canvas.draw_shape(&mut Line::new(self.x, 400, self.x + 10, 400));

        self.x = (self.x as i32 + self.step).clamp(0, canvas.width() as i32) as usize;

        if self.x >= canvas.width() - 10 || self.x < 1 {
            self.step = -self.step
        }
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
        step: 5,
        x: 0,
        y: 0,
    })?;
    Ok(())
}
