pub struct Vec3<T: Sized + Copy>{
    x: T,
    y: T,
    z: T,
}

impl<T: Sized+ Copy> From<(T,T,T)> for Vec3<T>{
    fn from(value: (T,T,T)) -> Self {
        let (x, y, z) = value;
        Vec3::new(x, y, z)
    }
}

impl<T: Sized+ Copy> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {x , y , z}
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
}