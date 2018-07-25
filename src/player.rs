use std::fmt;
use utils::Vec2;
use utils::Vec3;
use entity::{StatBlock, Facing, Attack, CombatResult, AttackType};

#[derive(Debug, Copy, Clone)]
pub enum Class {
    Warrior,
    Mage,
    Rogue
}

pub struct Player {
    name : String,
    pos : Vec2<usize>,
    base_stats : StatBlock,
    curr_stats : StatBlock,
    facing : Facing,
    target : Option<usize>
}

impl Player {
    pub fn new(name : String, pos : Vec2<usize>) -> Self {
        Player {
            name,
            pos,
            base_stats : StatBlock::new(10, 10, 10),
            curr_stats : StatBlock::new(10, 10, 10),
            facing : Facing::East,
            target : None
        }
    }

    pub fn set_position(&mut self, x : usize, y : usize) {
        self.pos.x = x;
        self.pos.y = y;
    }

    pub fn move_player(&mut self, new_pos : Vec2<usize>) {
        let x_dir = new_pos.x as i32 - self.pos.x as i32;
        let y_dir = new_pos.y as i32 - self.pos.y as i32;

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

        self.pos = new_pos;
    }

    pub fn send_attack(&self) -> Attack {
        let atk_pos = self.facing.position(self.pos);
        let damage = (self.curr_stats.strength * 3)/2;
        Attack::new(AttackType::Piercing, damage, atk_pos)
    }

    pub fn receive_attack(&mut self, attack : &Attack) -> CombatResult {
        self.curr_stats.health -= attack.damage;
        
        CombatResult {
            dmg_dealt : attack.damage,
            target_alive : self.alive(),
            target_name : self.name.clone()
        }
    }

    pub fn position(&self) -> &Vec2<usize> {
        &self.pos
    }

    pub fn collision(&self, other : Vec2<usize>) -> bool {
        other == self.pos
    }

    pub fn alive(&self) -> bool {
        self.curr_stats.health <= 0
    }

    pub fn base_stats(&self) -> &StatBlock {
        &self.base_stats
    }

    pub fn current_stats(&self) -> &StatBlock {
        &self.curr_stats
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn target(&self) -> Option<usize> {
        self.target
    }

    pub fn set_target(&mut self, uuid : usize) {
        self.target = Some(uuid);
    }

    pub fn clear_target(&mut self) {
        self.target = None;
    }
}


impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> x : {} y: {}", self.name, self.pos.x, self.pos.y)
    }
}
