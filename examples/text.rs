use graphics_rs::{
    color,
    graphics::Graphics,
    shapes::text::Text,
    simple_canvas::SimpleCanvas,
    traits::{canvas::Canvas, canvas_handler::CanvasHandler},
};

struct ShapesHandler;

impl<'a> CanvasHandler for ShapesHandler {
    fn update<T: Canvas>(&mut self, canvas: &mut T) {
        canvas.change_color(color::BLACK);
        canvas.fill();

        canvas.change_color(color::WHITE);
        canvas.draw_shape(&mut Text::new("A", 20, 0, 0));

        return;
    }
}

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
    graphics.run(&mut ShapesHandler {})?;

    Ok(())
}
