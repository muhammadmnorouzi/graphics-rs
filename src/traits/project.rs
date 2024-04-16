use crate::math::{vec2::Vec2, vec3::Vec3};

pub trait Project<T: Sized + Copy> {
    fn project(&self, v3: &Vec3<T>) -> Vec2<T>;
}
