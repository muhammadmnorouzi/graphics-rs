
const MIN: i64 = -10;
const MAX: i64 = -10;

use crate::{
    math::{vec2::Vec2, vec3::Vec3},
    traits::{
        canvas::Canvas,
        project::Project,
        shape::Shape,
    },
};

pub struct Cube3D<T: Sized + Copy + PartialOrd> {
    points: Vec<Vec3<T>>,
}

impl<T: Sized + Copy + PartialOrd> Cube3D<T> {
    pub fn new() {
        let mut points = Vec::<Vec3<i64>>::new();

        for x in MIN..MAX {
            for y in MIN..MAX {
                for z in MIN..MAX {
                    points.push((x, y, z).into());
                }
            }
        }
    }

    pub fn project_points(&self) -> Vec<Vec2<T>> {
        let mut projected_points = Vec::<Vec2<T>>::new();

        for point in &self.points {
            projected_points.push(Self::project(point))
        }

        projected_points
    }
}

impl<T: Sized + Copy + PartialOrd> Shape for Cube3D<T> {
    fn draw_to(&mut self, canvas: &mut impl Canvas) {
        let projected_points = self.project_points();

        for point in &projected_points {}
    }
}

impl<T: Sized + Copy + PartialOrd> Project for Cube3D<T> {
    fn project<S: Sized + Copy>(v3: &Vec3<S>) -> crate::math::vec2::Vec2<S> {
        Vec2::new(v3.x(), v3.y())
    }
}