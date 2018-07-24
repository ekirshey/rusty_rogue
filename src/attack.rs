use super::utils::math::Vec2;
use super::stats::StatBlock;
use super::world::World;
use super::player::Player;
use super::EntityMap;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ElementalType {
    Fire,
    Frost,
    Lightning
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AttackType {
    Slashing,
    Piercing,
    Elemental(ElementalType)
}

pub struct Attack {
    pub attack_type : AttackType,
    pub damage : i32,
    pub position : Vec2<usize>
}

pub struct CombatResult {
    pub dmg_dealt : i32,
    pub target_alive : bool,
    pub target_name : String
}

impl Attack {
    pub fn new( attack_type : AttackType, 
                damage : i32, 
                position : Vec2<usize>) -> Attack 
    {
        Attack {
            attack_type,
            damage,
            position
        }
    }
}

pub trait Attackable {
    fn update(&self, player : &Player, entities : &EntityMap, world : &World) -> Option<Attack>;
    fn receive_attack(&mut self, attack : &Attack) -> CombatResult; 
    fn collision(&self, other : Vec2<usize>) -> bool;

    // Getters
    fn position(&self) -> &Vec2<usize>;
    fn alive(&self) -> bool;
    fn base_stats(&self) -> &StatBlock;
    fn current_stats(&self) -> &StatBlock;
    fn name(&self) -> &str;
}