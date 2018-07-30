use std::collections::HashMap;
use utils::Vec2;
use utils::Vec3;
use entity::{DrawOutput};

pub struct Corpse {
    pos : Vec2<usize>
}
pub type CorpseMap = HashMap<usize, Corpse>;

impl Corpse {
    pub fn new(pos : Vec2<usize>) -> Corpse {
        Corpse {
            pos
        }
    }

    pub fn position(&self) -> &Vec2<usize> {
        &self.pos
    }

    pub fn draw(&self) -> DrawOutput {
        DrawOutput {
            position : self.pos,
            fg : Vec3::new(50, 50, 50),
            bg : Vec3::new(95,95,95),
            icon : 'c'
        }
    }
}
