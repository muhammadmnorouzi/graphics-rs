use std::{
    fmt::Debug,
    ops::{Add, Mul},
};

use crate::{
    math::{num_utils::NumUtils, vec2::Vec2, vec3::Vec3},
    traits::{canvas::Canvas, project::Project, shape::Shape},
};

use super::rect::Rect;

pub struct Cube3D<T>
where
    T: Sized + Copy + Clone + Add<Output = T> + Mul<Output = T> + Ord + Debug,
{
    points: Vec<Vec3<T>>,
    min: T,
    max: T,
    fov_factor: T,
    to_usize: fn(T) -> usize,
    from_usize: fn(usize) -> T,
}

impl<T> Cube3D<T>
where
    T: Sized + Copy + Clone + Add<Output = T> + Mul<Output = T> + Ord + Debug,
{
    pub fn new(
        points: Vec<Vec3<T>>,
        min: T,
        max: T,
        fov_factor: T,
        to_usize: fn(T) -> usize,
        from_usize: fn(usize) -> T,
    ) -> Self {
        assert!(min < max);

        Self {
            points,
            min,
            max,
            fov_factor,
            to_usize,
            from_usize,
        }
    }

    pub fn project_points(&self) -> Vec<Vec2<T>> {
        let mut projected_points = Vec::<Vec2<T>>::new();

        for point in &self.points {
            projected_points.push(self.project(point))
        }

        projected_points
    }
}

impl<T> Shape for Cube3D<T>
where
    T: Sized + Copy + Clone + Add<Output = T> + Mul<Output = T> + Ord + Debug,
{
    fn draw_to(&mut self, canvas: &mut impl Canvas) {
        let projected_points = self.project_points();

        for point in &projected_points {
            let (x, y) = (point.x(), point.y());
            let (x, y) = (x + self.min, y + self.min);
            let x = NumUtils::clamp(
                x,
                (self.from_usize)(usize::MIN),
                (self.from_usize)(usize::MAX),
            );
            let y = NumUtils::clamp(
                y,
                (self.from_usize)(usize::MIN),
                (self.from_usize)(usize::MAX),
            );

            let (x, y) = ((self.to_usize)(x), (self.to_usize)(y));

            canvas.draw_shape(&mut Rect::new(x, y, 5, 5));
        }
    }
}

impl<T> Project<T> for Cube3D<T>
where
    T: Sized + Copy + Clone + Add<Output = T> + Mul<Output = T> + Ord + Debug,
{
    fn project(&self, v3: &Vec3<T>) -> crate::math::vec2::Vec2<T> {
        Vec2::new(v3.x() * self.fov_factor, v3.y() * self.fov_factor)
    }
}
