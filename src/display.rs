use super::math::*;

pub struct DrawOutput {
    pub position : Vec2,
    pub fg : Vec3<u8>,
    pub bg : Vec3<u8>,
    pub icon : char
}

pub trait Drawable {
    fn draw(&self) -> DrawOutput;
}

// SOme draw manager that holds a list of Drawers
// A Drawer would be like GoblinDrawer 
// I would have a temporary list built up each step
// with a position and some meta information