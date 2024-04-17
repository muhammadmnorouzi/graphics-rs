#![allow(dead_code)]

mod color;
mod math;
mod shapes;
mod tools;
mod traits;

use graphics_rs::{
    graphics::Graphics,
    math::vec3::Vec3,
    shapes::point_cloud::PointCloud,
    simple_canvas::SimpleCanvas,
    tools::camera::Camera,
    traits::{canvas::Canvas, canvas_handler::CanvasHandler},
};

// Constants
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

struct MyCanvasHandler {
    point_cloud: PointCloud,
}

impl<'a> CanvasHandler for MyCanvasHandler {
    fn update<T: Canvas>(&mut self, canvas: &mut T) {
        canvas.change_color(color::BLACK);
        canvas.fill();
        canvas.change_color(color::GREEN);
        canvas.draw_shape(&mut self.point_cloud);
        return;
    }
}

fn main() -> Result<(), String> {
    let mut canvas =
        SimpleCanvas::new(WIDTH as usize, HEIGHT as usize, Some(color::BLACK), true, 4);

    let mut graphics = Graphics::create(&mut canvas)?;
    graphics.run(&mut MyCanvasHandler {
        point_cloud: create_point_cloud(),
    })?;
    Ok(())
}

fn create_point_cloud() -> PointCloud {
    let min = -10;
    let max = 10;
    let fov_factor = 200;
    let mut points = Vec::<Vec3<i64>>::new();

    for x in min..max {
        for y in min..max {
            for z in min..max {
                points.push(Vec3::new(x, y, z));
            }
        }
    }

    let camera = Camera::new(Vec3::new(0, 0, -20), Vec3::new(0, 0, 0), 90);

    PointCloud::new(points, min, max, fov_factor, camera , 1)
}
