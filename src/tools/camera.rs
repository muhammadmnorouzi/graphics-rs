use crate::math::vec3::Vec3;

pub struct Camera<T: Sized + Copy> {
    position: Vec3<T>,
    direction: Vec3<T>,
    fov_angle: T,
}

impl<T> Camera<T>
where
    T: Sized + Copy,
{
    pub fn new(position: Vec3<T>, direction: Vec3<T>, fov_angle: T) -> Self {
        Self {
            position,
            direction,
            fov_angle,
        }
    }

    pub fn position(&self) -> &Vec3<T> {
        &self.position
    }
}
