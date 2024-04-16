use crate::math::{vec2::Vec2, vec3::Vec3};

pub trait Project {
    fn project<T: Sized + Copy>(v3: &Vec3<T>) -> Vec2<T>;
}
