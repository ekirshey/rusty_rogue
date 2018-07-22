use super::math::Vec2;
use super::math::Vec3;
use super::attack::*;
use super::stats::*;
use super::display::*;

pub struct Goblin {
    pos : Vec2,
    base_stats : StatBlock,
    curr_stats : StatBlock,
    facing : Facing,
    alive : bool
}

impl Goblin {
    pub fn new(pos : Vec2) -> Goblin {
        Goblin {
            pos,
            base_stats : StatBlock::new(30,10,10),
            curr_stats : StatBlock::new(30,10,10),
            facing : Facing::North,
            alive : true
        }
    }
}

impl Attackable for Goblin {
    fn send_attack(&self) -> Attack {
        let atk_pos = facing_position(self.facing, self.pos);
        let damage = (self.curr_stats.strength * 3)/2;
        Attack::new(AttackType::Piercing, damage, atk_pos)
    }

    fn receive_attack(&mut self, attack : &Attack) -> CombatResult {
        self.curr_stats.health -= attack.damage;
        if self.curr_stats.health <= 0 {
            self.alive = false;
        }
        
        CombatResult {
            dmg_dealt : attack.damage,
            target_alive : self.alive,
            target_name : String::from("a Goblin")
        }
    }

    fn position(&self) -> &Vec2 {
        &self.pos
    }

    fn collision(&self, other : Vec2) -> bool {
        self.alive && other == self.pos
    }

    fn alive(&self) -> bool {
        self.alive
    }
}

impl Drawable for Goblin {
    fn draw(&self) -> DrawOutput {
        let percent_health : f32 = self.curr_stats.health as f32 /self.base_stats.health as f32;
        let red = (255.0 * (1.0-percent_health)) as u8;
        let green = (255.0 *(percent_health)) as u8;
        DrawOutput {
            position : self.pos,
            fg : Vec3::new(red, green, 0),
            bg : Vec3::new(95,95,95),
            icon : 'g'
        }
    }
}