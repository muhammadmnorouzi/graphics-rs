use graphics_rs::{
    color,
    graphics::Graphics,
    shapes::{circle::Circle, line::Line, rect::Rect},
    simple_canvas::SimpleCanvas,
    traits::{canvas::Canvas, canvas_handler::CanvasHandler},
};

struct ShapesHandler;

impl<'a> CanvasHandler for ShapesHandler {
    fn update<T: Canvas>(&mut self, canvas: &mut T) {
        canvas.change_color(color::BLACK);
        canvas.fill();

        canvas.change_color(color::GREEN);
        canvas.draw_shape(&mut Circle::new(100, 400, 200));

        // canvas.change_color(color::BLUE);
        // canvas.draw_shape(&mut Line::new(400, 0, 400, 800));
        // canvas.draw_shape(&mut Line::new(0, 0, canvas.width(), canvas.height()));
        // canvas.draw_shape(&mut Line::new(0, 400, 800, 400));
        // canvas.draw_shape(&mut Line::new(0, canvas.height(), canvas.width(), 0));

        // canvas.change_color(color::RED);
        // canvas.draw_shape(&mut Rect::new(300, 200, 200, 100));
        // canvas.draw_shape(&mut Rect::new(200, 300, 100, 200));
        // canvas.draw_shape(&mut Rect::new(300, 500, 200, 100));
        // canvas.draw_shape(&mut Rect::new(500, 300, 100, 200));

        // canvas.change_color(color::GREEN);
        // canvas.draw_shape(&mut Circle::new(200, 200, 50));
        // canvas.draw_shape(&mut Circle::new(200, 600, 50));
        // canvas.draw_shape(&mut Circle::new(600, 200, 50));
        // canvas.draw_shape(&mut Circle::new(600, 600, 50));

        return;
    }
}

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn main() -> Result<(), String> {
    let antialiasing = true;
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
