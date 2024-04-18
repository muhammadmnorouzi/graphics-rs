use std::clone;

#[derive(Clone, Copy)]
pub struct Vec3<T: Sized + Copy> {
    x: T,
    y: T,
    z: T,
}

impl<T: Sized + Copy> From<(T, T, T)> for Vec3<T> {
    fn from(value: (T, T, T)) -> Self {
        let (x, y, z) = value;
        Vec3::new(x, y, z)
    }
}

impl<T: Sized + Copy> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }

    pub fn z(&self) -> T {
        self.z
    }

    pub fn set_y(&mut self, y: T) {
        self.y = y;
    }

    pub fn set_x(&mut self, x: T) {
        self.x = x;
    }

    pub fn set_z(&mut self, z: T) {
        self.z = z;
    }

    pub fn set(&mut self, x: T, y: T, z: T) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
}

impl Vec3<i64> {
    pub fn rotate_x(&self, degrees: f64) -> Vec3<i64> {
        let cos = f64::cos(degrees.to_radians());
        let sin = f64::sin(degrees.to_radians());

        let x = (self.y as f64 * cos - self.z as f64 * sin) as i64;
        let z = (self.y as f64 * sin + self.z as f64 * cos) as i64;

        Vec3::<i64>::new(self.x, x, z)
    }

    pub fn rotate_y(&self, degrees: f64) -> Vec3<i64> {
        let cos = f64::cos(degrees.to_radians());
        let sin = f64::sin(degrees.to_radians());

        let x = (self.x as f64 * cos - self.z as f64 * sin) as i64;
        let z = (self.x as f64 * sin + self.z as f64 * cos) as i64;

        Vec3::<i64>::new(x, self.y, z)
    }

    pub fn rotate_z(&self, degrees: f64) -> Vec3<i64> {
        let cos = f64::cos(degrees.to_radians());
        let sin = f64::sin(degrees.to_radians());

        let x = (self.x as f64 * cos - self.y as f64 * sin) as i64;
        let y = (self.x as f64 * sin + self.y as f64 * cos) as i64;

        Vec3::<i64>::new(x, y, self.z)
    }

    pub fn rotate(&self, rotation: &Vec3<f64>) -> Vec3<i64> {
        if rotation.magnitude() > 0f64 {
            self.rotate_x(rotation.x)
            .rotate_y(rotation.y)
            .rotate_z(rotation.z)
        }else {
            return self.clone()
        }
    }
}

impl Vec3<f64> {
    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x.powf(2f64) + self.y.powf(2f64) + self.y.powf(2f64))
    }
}
