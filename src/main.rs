#![allow(dead_code)]

mod color;
mod math;
mod shapes;
mod tools;
mod traits;

use graphics_rs::{
    graphics::Graphics,
    math::vec3::Vec3,
    shapes::cube3d::Cube3D,
    simple_canvas::SimpleCanvas,
    traits::{canvas::Canvas, canvas_handler::CanvasHandler},
};

// Constants
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

struct MyCanvasHandler {
    cube: Cube3D<u64>,
}

impl<'a> CanvasHandler for MyCanvasHandler {
    fn update<T: Canvas>(&mut self, canvas: &mut T) {
        canvas.change_color(color::BLACK);
        canvas.fill();
        canvas.change_color(color::GREEN);
        canvas.draw_shape(&mut self.cube);
        return;
    }
}

fn main() -> Result<(), String> {
    let mut canvas =
        SimpleCanvas::new(WIDTH as usize, HEIGHT as usize, Some(color::BLACK), true, 4);

    let mut graphics = Graphics::create(&mut canvas)?;
    graphics.run(&mut MyCanvasHandler {
        cube: create_cube(),
    })?;
    Ok(())
}

fn create_cube() -> Cube3D<u64> {
    let min = 0;
    let max = 10;
    let fov_factor = 20;
    let mut points = Vec::<Vec3<u64>>::new();

    for x in min..max {
        for y in min..max {
            for z in min..max {
                points.push(Vec3::new(x, y, z));
            }
        }
    }

    Cube3D::new(points, min, max, fov_factor, |x| x as usize, |x| x as u64)
}
