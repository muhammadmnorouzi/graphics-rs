use crate::{math::vec3::Vec3, traits::{canvas::Canvas, shape::Shape}};

pub struct Cube3D {
    points: Vec<Vec3<f64>>,
}

impl Cube3D {
    pub fn new(){
        let mut points = Vec::<Vec3<f64>>::new();

        for x in -10..10 {
            for y in -10..10 {
                for z in -10..10 {
                    let a = 10f64;
                    let (x , y , z) = (x as f64 / a, y as f64/ a, z as f64/ a);
                    points.push((x , y , z).into());
                }
            }
        }
    }
}

impl Shape for Cube3D {
    fn draw_to(&mut self, canvas: &mut impl Canvas) {
    }
}
