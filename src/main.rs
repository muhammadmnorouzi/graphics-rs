mod canvas;
mod color;

use graphics_rs::{
    canvas::Canvas,
    graphics::{Graphics, GraphicsHandler},
};

struct CustomeHandler {
    rate: usize,
    rect: (usize, usize, usize, usize),
}

impl GraphicsHandler for CustomeHandler {
    fn draw(&mut self, canvas: &mut Canvas) {
        canvas.change_color(color::BLACK);
        canvas.fill();
        canvas.change_color(color::GREEN);
        canvas.fill_rect(self.rect.0 ,self.rect.1 , self.rect.2 , self.rect.3);

        self.rect.2 += self.rate;
        self.rect.3 += self.rate;

        if self.rect.2 > canvas.width(){
            self.rect.2 = 100;
        }

        if self.rect.3 > canvas.height(){
            self.rect.3 = 100;
        }
    }
}

// Constants
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn main() -> Result<(), String> {
    let canvas = Canvas::create(WIDTH as usize, HEIGHT as usize, None);
    let mut graphics = Graphics::create(canvas);
    graphics.run(&mut CustomeHandler { rate: 5, rect: (0 , 0 , 100 , 100)})?;
    Ok(())
}
