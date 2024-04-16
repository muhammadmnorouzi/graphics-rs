pub struct Vec3<T: Sized + Copy>{
    x: T,
    y: T,
    z: T,
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