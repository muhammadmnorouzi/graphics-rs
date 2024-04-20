use graphics_rs::{color, graphics::Graphics, shapes::{line::Line, point_cloud::PointCloud}, simple_canvas::SimpleCanvas, traits::{canvas::Canvas, canvas_handler::CanvasHandler}};

struct ShapesHandler;

impl<'a> CanvasHandler for ShapesHandler {
    fn update<T: Canvas>(&mut self, canvas: &mut T) {
        canvas.change_color(color::BLACK);
        canvas.fill();

        canvas.change_color(color::BLUE);
        canvas.draw_shape(&mut Line::new(canvas.width() / 2, 0, canvas.width() / 2, canvas.height()));
        canvas.draw_shape(&mut Line::new(0, 0, canvas.width() , canvas.height()));
        canvas.draw_shape(&mut Line::new(0, canvas.height() / 2, canvas.width() , canvas.height() / 2));
        canvas.draw_shape(&mut Line::new(0, canvas.height(),  canvas.width(), 0));

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