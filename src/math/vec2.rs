pub struct Vec2<T: Sized + Copy>{
    x: T,
    y: T
}

impl<T: Sized+ Copy> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {x , y}
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }
}