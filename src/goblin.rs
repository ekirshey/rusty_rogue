use std::ops::BitXor;
use utils::Vec2;
use utils::Vec3;
use world::Tile;
use entity::{Attackable, Attack, AttackType, CombatResult, 
             StatBlock, Facing, Drawable, DrawOutput};
use super::player::Player;


pub struct Goblin {
    pos : Vec2<usize>,
    base_stats : StatBlock,
    curr_stats : StatBlock,
    facing : Facing,
    alive : bool
}

impl Goblin {
    pub fn new(pos : Vec2<usize>) -> Goblin {
        Goblin {
            pos,
            base_stats : StatBlock::new(2,2,2),
            curr_stats : StatBlock::new(2,2,2),
            facing : Facing::North,
            alive : true
        }
    }
}

impl Attackable for Goblin {
    fn update(&mut self, player : &Player, tiles : &Vec<Tile>, room_size : Vec2<usize>) -> Option<Attack> {
        let m_pos = Vec2::new(self.position().x  as i32, self.position().y  as i32);
        let p_pos = Vec2::new(player.position().x as i32, player.position().y as i32); 
        let diff = p_pos - m_pos;
        if diff.x.abs().bitxor(diff.y.abs()) == 1 {
            let atk_pos = self.facing.position(self.pos);
            let damage = self.curr_stats.strength;
            return Some(Attack::new(AttackType::Piercing, damage, atk_pos));
        }
        else {
            let mut n_x = 0;
            let mut n_y = 0;

            if diff.x != 0 {
                n_x = diff.x/diff.x.abs();
            }
            if diff.y != 0 {
                n_y = diff.y/diff.y.abs();
            }

            let normal = Vec2::new(n_x, n_y);
            let new_pos = m_pos + normal;

            if diff.y.abs() > diff.x.abs() &&
               new_pos.y > 0 && 
               !tiles[self.pos.x + new_pos.y as usize * room_size.x].occupied 
            {
                self.pos.y = new_pos.y as usize;
            }
            else if new_pos.x > 0 && 
               !tiles[new_pos.x as usize + self.pos.y * room_size.x].occupied 
            {
                self.pos.x = new_pos.x as usize;
            }
        }

        None
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

    fn position(&self) -> &Vec2<usize> {
        &self.pos
    }

    fn collision(&self, other : Vec2<usize>) -> bool {
        self.alive && other == self.pos
    }

    fn alive(&self) -> bool {
        self.alive
    }

    fn base_stats(&self) -> &StatBlock {
        &self.base_stats
    }

    fn current_stats(&self) -> &StatBlock {
        &self.curr_stats
    }

    fn name(&self) -> &str {
        "a Goblin"
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