use super::math::Vec2;

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
    pub position : Vec2
}

pub struct CombatResult {
    pub dmg_dealt : i32,
    pub target_alive : bool,
    pub target_name : String
}

impl Attack {
    pub fn new( attack_type : AttackType, 
                damage : i32, 
                position : Vec2) -> Attack 
    {
        Attack {
            attack_type,
            damage,
            position
        }
    }
}

pub trait Attackable {
    fn send_attack(&self) -> Attack;
    fn receive_attack(&mut self, attack : &Attack) -> CombatResult; 
    fn position(&self) -> &Vec2;
    fn collision(&self, other : Vec2) -> bool;
    fn alive(&self) -> bool;
}