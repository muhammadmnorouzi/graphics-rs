use crate::math::vec3::Vec3;

pub struct Camera<T: Sized + Copy> {
    position: Vec3<T>,
    direction: Vec3<T>,
    fov_angle: T,
}
