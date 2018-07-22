use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x : i32,
    pub y : i32
}

impl Vec2 {
    pub fn new(x : i32, y : i32) -> Vec2 {
        Vec2 {
            x,
            y
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x : self.x + other.x,
            y : self.y + other.y,
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
