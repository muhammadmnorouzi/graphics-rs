use crate::{
    math::{vec2::Vec2, vec3::Vec3},
    tools::camera::Camera,
    traits::{canvas::Canvas, project::Project, shape::Shape},
};

use super::rect::Rect;

pub struct PointCloud {
    points: Vec<Vec3<i64>>,
    min: i64,
    max: i64,
    fov_factor: i64,
    camera: Camera<i64>,
    rotation: Vec3<f64>,
    size: usize,
}

impl PointCloud {
    pub fn new(
        points: Vec<Vec3<i64>>,
        min: i64,
        max: i64,
        fov_factor: i64,
        camera: Camera<i64>,
        rotation: Vec3<f64>,
        size: usize,
    ) -> Self {
        Self {
            points,
            min,
            max,
            fov_factor,
            camera,
            rotation,
            size,
        }
    }

    pub fn project_points(&self) -> Vec<Vec2<i64>> {
        let mut projected_points = Vec::<Vec2<i64>>::new();

        for point in &self.points {
            if let Some(point) = self.project(&point.rotate(&self.rotation)) {
                projected_points.push(point);
            }
        }

        projected_points
    }

    pub fn set_rotation(&mut self, rotation: Vec3<f64>) {
        self.rotation = rotation
    }

    pub fn rotation(&mut self) -> &mut Vec3<f64> {
        &mut self.rotation
    }
}

impl Shape for PointCloud {
    fn draw_to(&mut self, canvas: &mut impl Canvas) {
        let projected_points = self.project_points();

        for point in &projected_points {
            let (x, y) = (point.x(), point.y());

            let (x, y) = (
                x + canvas.width() as i64 / 2,
                y + canvas.height() as i64 / 2,
            );

            if x >= 0 && x <= canvas.width() as i64 && y >= 0 && y <= canvas.height() as i64 {
                let x = x.clamp(usize::MIN as i64, i64::MAX);
                let y = y.clamp(usize::MIN as i64, i64::MAX);

                let half_size = self.size as i64 / 2;
                canvas.draw_shape(&mut Rect::new(
                    x + half_size,
                    y + half_size,
                    self.size,
                    self.size,
                ));
            }
        }
    }
}

impl Project<i64> for PointCloud {
    fn project(&self, v3: &Vec3<i64>) -> Option<Vec2<i64>> {
        let z = v3.z() - self.camera.position().z();
        if z == 0 {
            None
        } else {
            let x = v3.x() as f64 * self.fov_factor as f64 / z as f64;
            let y = v3.y() as f64 * self.fov_factor as f64 / z as f64;

            let (x, y) = (x as i64, y as i64);

            Some(Vec2::new(x, y))
        }
    }
}
