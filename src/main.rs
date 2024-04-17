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
const WIDTH: usize = 800;
const HEIGHT: usize = 800;

struct MyCanvasHandler {
    point_cloud: PointCloud,
}

impl<'a> CanvasHandler for MyCanvasHandler {
    fn update<T: Canvas>(&mut self, canvas: &mut T) {
        canvas.change_color(color::BLACK);
        canvas.fill();

        canvas.change_color(color::RED);
        canvas.draw_shape(&mut self.point_cloud);

        let rotation = self.point_cloud.rotation();
        rotation.set_y(rotation.y() + 0.5f64);

        return;
    }
}

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
    graphics.run(&mut MyCanvasHandler {
        point_cloud: create_point_cloud(),
    })?;

    Ok(())
}

fn create_point_cloud() -> PointCloud {
    let min = -6;
    let max = 6;
    let fov_factor = 350;
    let mut points = Vec::<Vec3<i64>>::new();
    let size = 2;

    for x in min..max {
        for y in min..max {
            for z in min..max {
                points.push(Vec3::new(x, y, z));
            }
        }
    }

    let camera = Camera::new(Vec3::new(0, 0, -20), Vec3::new(0, 0, 0), 90);

    PointCloud::new(
        points,
        min,
        max,
        fov_factor,
        camera,
        Vec3::<f64>::new(0f64, 0f64, 0f64),
        size,
    )
}
