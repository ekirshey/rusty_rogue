use std::fmt;
use super::math::Vec2;
use super::math::Vec3;
use super::attack::*;
use super::display::*;
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
    base_stats : StatBlock,
    curr_stats : StatBlock,
    facing : Facing
}

impl Player {
    pub fn new(name : String, pos : Vec2<usize>) -> Self {
        Player {
            name,
            pos,
            base_stats : StatBlock::new(10, 10, 10),
            curr_stats : StatBlock::new(10, 10, 10),
            facing : Facing::East
        }
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

}

impl Attackable for Player {
    fn send_attack(&self) -> Attack {
        let atk_pos = facing_position(self.facing, self.pos);
        let damage = (self.curr_stats.strength * 3)/2;
        Attack::new(AttackType::Piercing, damage, atk_pos)
    }

    fn receive_attack(&mut self, attack : &Attack) -> CombatResult {
        self.curr_stats.health -= attack.damage;
        
        CombatResult {
            dmg_dealt : attack.damage,
            target_alive : self.alive(),
            target_name : self.name.clone()
        }
    }

    fn position(&self) -> &Vec2<usize> {
        &self.pos
    }

    fn collision(&self, other : Vec2<usize>) -> bool {
        other == self.pos
    }

    fn alive(&self) -> bool {
        self.curr_stats.health <= 0
    }

    fn base_stats(&self) -> &StatBlock {
        &self.base_stats
    }

    fn current_stats(&self) -> &StatBlock {
        &self.curr_stats
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Drawable for Player {
    fn draw(&self) -> DrawOutput {
        let percent_health : f32 = self.curr_stats.health as f32 /self.base_stats.health as f32;
        let red = (255.0 * (1.0-percent_health)) as u8;
        let green = (255.0 *(percent_health)) as u8;
        DrawOutput {
            position : self.pos,
            fg : Vec3::new(red, green, 0),
            bg : Vec3::new(95,95,95),
            icon : '@'
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> x : {} y: {}", self.name, self.pos.x, self.pos.y)
    }
}
