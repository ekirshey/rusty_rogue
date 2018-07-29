extern crate rand;

use utils::Vec2;
use world::{Cell, CellType};
use entity::{EntityMap};
use player::Player;
use log::Log;
use world::Direction;
use std::collections::HashMap;

use self::rand::prelude::*;

// Entities
use goblin::Goblin;

struct Entrance {
    pub location : Vec2<usize>,
    pub direction : Direction
}

pub struct Room {
    size : Vec2<usize>,
    init_pos : Vec2<usize>,
    tiles : Vec<Cell>,
    entrances : Vec<Entrance>,
    entities : EntityMap
}

impl Room {
    pub fn new(size : Vec2<usize>) -> Room {
        let c = Cell::new();

        let mut tiles = vec![c; size.x*size.y];
        for i in 0..tiles.len() {
            let x = i % size.x;
            let y = i / size.x;
            if x != 0 && y != 0 && x != size.x-1 && y != size.y-1 {
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
            entrances : Vec::new(),
            entities
        }
    }

    pub fn entering_position(&self, entering_direction : Direction) -> Vec2<usize> {
        for entrance in &self.entrances {
            if entrance.direction == entering_direction {
                return entrance.location;
            }
        }

        panic!("Tried entering a room without a valid entrance");
    }

    pub fn add_neighbor(&mut self, direction : Direction, neighbor_id : usize) {
        let exit_cell = CellType::Exit{
            node_id : neighbor_id,
            exiting_direction : direction
        };

        let size = self.size;
        let mut position : Vec2<usize>;
        match direction {
            Direction::North => {
                let rng = rand::thread_rng().gen_range(1, size.x-1);
                position = Vec2::new(rng, 0);
                self.get_cell_mut(position).id = exit_cell;
                self.entrances.push(Entrance {
                    location : Vec2::new(position.x, position.y+1),
                    direction : direction
                });
            },
            Direction::East => {
                let rng = rand::thread_rng().gen_range(1, size.y-1);
                position = Vec2::new(size.x-1, rng);
                self.get_cell_mut(position).id = exit_cell;
                self.entrances.push(Entrance {
                    location : Vec2::new(position.x-1, position.y),
                    direction : direction
                });
            },
            Direction::South => {
                let rng = rand::thread_rng().gen_range(1, size.x-1);
                position = Vec2::new(rng, size.y-1);
                self.get_cell_mut(position).id = exit_cell;
                self.entrances.push(Entrance {
                    location : Vec2::new(position.x, position.y-1),
                    direction : direction
                });
            },
            Direction::West => {
                let rng = rand::thread_rng().gen_range(1, size.y-1);
                position = Vec2::new(0, rng);
                self.get_cell_mut(position).id = exit_cell;
                self.entrances.push(Entrance {
                    location : Vec2::new(position.x+1, position.y),
                    direction : direction
                });
            }
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
            let result = self.tiles.get(pos.x + pos.y * self.size.x);
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
                                log : &mut Log) -> CellType
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

        self.get_cell_type(new_pos)
    }

    fn get_cell_mut(&mut self, loc : Vec2<usize>) -> &mut Cell {
        &mut self.tiles[loc.x + loc.y * self.size.x]
    }

    fn get_cell_type(&self, loc : Vec2<usize>) -> CellType {
        self.tiles[loc.x + loc.y * self.size.x].id
    }
}