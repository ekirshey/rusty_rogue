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
    pos : Vec2,
    stats : StatBlock,
    facing : Facing
}

impl Player {
    pub fn new(name : String, pos : Vec2) -> Self {
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

    pub fn set_position(&mut self, x : i32, y : i32) {
        self.pos.x = x;
        self.pos.y = y;
    }

    pub fn move_player(&mut self, x_dir : i32, y_dir : i32) {
        self.pos.y += y_dir;
        self.pos.x += x_dir;

        if x_dir > 0 {
            self.facing = Facing::East;
        }

        if x_dir < 0 {
            self.facing = Facing::West;
        }

        if y_dir > 0 {
            self.facing = Facing::North;
        }
        
        if y_dir < 0 {
            self.facing = Facing::South;
        }
        
    }

    pub fn position(&self) -> Vec2 {
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
