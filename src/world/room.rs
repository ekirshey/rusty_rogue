use utils::Vec2;
use world::{Cell, CellType};
use entity::{EntityMap};
use player::Player;
use log::Log;
use std::collections::HashMap;

// Entities
use goblin::Goblin;

pub struct Room {
    size : Vec2<usize>,
    init_pos : Vec2<usize>,
    tiles : Vec<Cell>,
    entities : EntityMap
}

impl Room {
    pub fn new(size : Vec2<usize>) -> Room {
        let c = Cell::new();

        let mut tiles = vec![c; size.x*size.y];
        for i in 0..tiles.len() {
            let x = i % size.x;
            let y = i / size.x;
            if x != 0 && y != 0 && x != size.x-1 && y != size.y-1{
                tiles[i].id = CellType::Granite;
            }
        }

        // Need a global static uuid thing
        let mut uuid = 0;

        let mut entities : EntityMap = HashMap::new();
        for i in 0..4 {
            let bc = Box::new(Goblin::new(Vec2::new(5+i,5+i)));
            entities.insert(uuid, bc);
            uuid += 1;
        }

        Room {
            size,
            init_pos : Vec2::new(1,1),
            tiles,
            entities
        }
    }

    pub fn tiles(&self) -> &Vec<Cell> {
        &self.tiles
    }

    pub fn width(&self) -> usize {
        self.size.x
    }

    pub fn height(&self) -> usize {
        self.size.y
    }

    pub fn initial_position(&self) -> Vec2<usize> {
        self.init_pos
    }

    pub fn valid_position(&self, pos : Vec2<usize> ) -> bool {
        if pos.x < self.size.x && pos.y < self.size.y {
            let result = self.tiles.get(pos.x + pos.y * self.size.y);
            if let Some(cell) = result {
                return !cell.id.collidable();
            }
            else {
                return false;
            }
        }

        false
    }

    pub fn step(&mut self, player : &mut Player) {
        let mut rem = Vec::new();

        for (uuid, m) in &mut self.entities {
            if !m.alive() {
                rem.push(*uuid);
                if let Some(target_id) = player.target() {
                    if target_id == *uuid {
                        player.clear_target();
                    }
                }
            }
        }
        
        for uuid in &rem {
            self.entities.remove(uuid);
        }
    }

    pub fn get_entities(&self) -> &EntityMap {
        &self.entities
    }

    pub fn get_mut_entities(&mut self) -> &mut EntityMap {
        &mut self.entities
    }

    pub fn handle_player_input( &mut self, 
                                player : &mut Player,
                                new_pos : Vec2<usize>,
                                log : &mut Log) 
    {
        let mut blocked = false;
        for (uuid, mut m) in &mut self.entities {
            if m.collision(new_pos) {
                let attack = player.send_attack();
                let result = m.receive_attack(&attack);
                log.log_combat(&player, &result);
                player.set_target(*uuid);
                if result.target_alive {
                    blocked = true;
                }
            }
        }

        if !blocked && self.valid_position(new_pos) {
            player.move_player(new_pos);
        }
    }
}