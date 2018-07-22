use std::fmt;
use super::math::Vec2;
use super::attack::*;
use super::stats::*;

#[derive(Debug, Copy, Clone)]
pub enum Class {
    Warrior,
    Mage,
    Rogue
}

pub struct Player {
    name : String,
    pos : Vec2<usize>,
    stats : StatBlock,
    facing : Facing
}

impl Player {
    pub fn new(name : String, pos : Vec2<usize>) -> Self {
        Player {
            name,
            pos,
            stats : StatBlock::new(10, 10, 10),
            facing : Facing::East
        }
    }

    pub fn send_attack(&self) -> Attack {
        let atk_pos = facing_position(self.facing, self.pos);
        let damage = (self.stats.strength * 3)/2;
        Attack::new(AttackType::Piercing, damage, atk_pos)
    }

    pub fn set_position(&mut self, x : usize, y : usize) {
        self.pos.x = x;
        self.pos.y = y;
    }

    pub fn move_player(&mut self, x_dir : i32, y_dir : i32) {
        let mut lcl_x = x_dir;
        let mut lcl_y = y_dir;
        if (self.pos.x as i32 + x_dir) < 0 {
            lcl_x = 0;
        }

        if (self.pos.y as i32 + y_dir) < 0 {
            lcl_y = 0;
        }

        self.pos.x = (self.pos.x as i32 + lcl_x) as usize;
        self.pos.y = (self.pos.y as i32 + lcl_y) as usize;

        if lcl_x > 0 {
            self.facing = Facing::East;
        }

        if lcl_x < 0 {
            self.facing = Facing::West;
        }

        if lcl_y > 0 {
            self.facing = Facing::North;
        }
        
        if lcl_y < 0 {
            self.facing = Facing::South;
        }
        
    }

    pub fn position(&self) -> Vec2<usize> {
        self.pos
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> x : {} y: {}", self.name, self.pos.x, self.pos.y)
    }
}
