use std::ops::Add;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2<T> {
    pub x : T,
    pub y : T
}

impl<T> Vec2<T> {
    pub fn new(x : T, y : T) -> Vec2<T> {
        Vec2 {
            x,
            y
        }
    }
}

impl<T: Add<Output=T>> Add for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, other: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x : self.x + other.x,
            y : self.y + other.y
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3<T> {
    pub x : T,
    pub y : T,
    pub z : T,
}

impl<T> Vec3<T> {
    pub fn new(x : T, y : T, z : T) -> Vec3<T> {
        Vec3 {
            x,
            y,
            z,
        }
    }
}

impl<T: Add<Output=T>> Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x : self.x + other.x,
            y : self.y + other.y,
            z : self.z + other.z,
        }
    }
}
