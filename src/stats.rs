use math::Vec2;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Facing {
    North,
    East,
    South,
    West
}

pub fn facing_position(facing : Facing, pos : Vec2<usize>) -> Vec2<usize> {
    match facing {
        Facing::North => Vec2::new(pos.x, pos.y + 1),
        Facing::East => Vec2::new(pos.x + 1, pos.y),
        Facing::South => Vec2::new(pos.x, pos.y - 1),
        Facing::West => Vec2::new(pos.x - 1, pos.y)
    }
}


pub struct StatBlock {
    pub strength : i32,
    pub dexterity : i32,
    pub intelligence : i32,
    pub health : i32,
    pub mana : i32
}

impl StatBlock {
    pub fn new(strength : i32, dexterity : i32, intelligence : i32) -> Self {
        StatBlock {
            strength,
            dexterity,
            intelligence,
            health : strength * 2,
            mana : intelligence * 2
        }
    }
}