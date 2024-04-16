use std::{char::MAX, collections::btree_map::Range, iter::Step, ops::{Add, Mul}};

use crate::{
    math::{vec2::Vec2, vec3::Vec3},
    traits::{canvas::Canvas, project::Project, shape::Shape},
};

use super::rect::Rect;

pub struct Cube3D<T: Sized + Copy + Clone + Add + Mul> {
    points: Vec<Vec3<T>>,
    min: T,
    max: T,
    fov_factor: T
}

impl<T: Sized + Copy + Clone+ Add+ Mul> Cube3D<T> {
    pub fn new(min: T , max: T, fov_factor: T) -> Self {
        let mut points = Vec::<Vec3<T>>::new();

        for x in min..max {
            for y in  MIN..MAX {
                for z in MIN..MAX {
                    points.push((x, y, z));
                }
            }
        }

        Self { points , fov_factor}
    }

    pub fn project_points(&self) -> Vec<Vec2<T>> {
        let mut projected_points = Vec::<Vec2<T>>::new();

        for point in &self.points {
            projected_points.push(self.project(point))
        }

        projected_points
    }
}

impl<T: Sized + Copy + Clone + Add + Mul> Shape for Cube3D<T> {
    fn draw_to(&mut self, canvas: &mut impl Canvas) {
        let projected_points = self.project_points();

        for point in &projected_points {
            let (x, y) = (point.x(), point.y());
            let (x, y) = (x + self.min, y + self.min);
            let (x , y) = (x.clamp())
            let (x, y) = (usize::from(x), y as usize);

            println!("x:{} y:{}", x, y);

            canvas.draw_shape(&mut Rect::new(x, y, 100, 100));
        }
    }
}

impl<T: Sized + Copy + Clone + Add + Mul> Project<T> for Cube3D<T> {
    fn project(&self, v3: &Vec3<T>) -> crate::math::vec2::Vec2<T> {
        Vec2::new(v3.x() * self.fov_factor, v3.y() * self.fov_factor)
    }
}
