extern crate rand;

use std::collections::HashMap;
use std::borrow::BorrowMut;
use self::rand::prelude::*;

use utils::Vec2;
use world::{Tile, TileType};
use entity::{EntityMap, Entity, CorpseMap, Corpse, Attack};
use player::Player;
use log::Log;
use world::Direction;

// Entities
use goblin::Goblin;

struct Entrance {
    pub location : Vec2<usize>,
    pub direction : Direction
}

pub struct RoomProperties {
    // theme
    // "Difficulty" 
    //// possible monsters
    //// amount of monsters
}

pub struct Room {
    size : Vec2<usize>,
    init_pos : Vec2<usize>,
    tiles : Vec<Tile>,
    entrances : Vec<Entrance>,
    entities : EntityMap,
    corpses : CorpseMap
}

impl Room {
    pub fn new(size : Vec2<usize>) -> Room {
        let mut tiles = Vec::new();

        for i in 0..size.x*size.y {
            let x = i % size.x;
            let y = i / size.x;
            if x != 0 && y != 0 && x != size.x-1 && y != size.y-1 {
                tiles.push(Tile::new(TileType::Granite));
            }
            else {
                tiles.push(Tile::new(TileType::Wall));
            }
        }

        // Need a global static uuid thing
        let mut uuid = 0;

        let mut entities : EntityMap = HashMap::new();
        // Max range should be based on area I think
        let available_space = size.x * size.y - (size.y *2) - (size.x-2 * 2);
        let num_gobbos = rand::thread_rng().gen_range(1, available_space/3);
        for i in 0..num_gobbos {
            let mut pos = Vec2::new(rand::thread_rng().gen_range(2, size.x-1),
                                rand::thread_rng().gen_range(2, size.y-1));
            
            while tiles[pos.x + pos.y * size.x].occupied {
                pos = Vec2::new(rand::thread_rng().gen_range(2, size.x-1),
                                rand::thread_rng().gen_range(2, size.y-1));    
            }

            tiles[pos.x + pos.y * size.x].occupied = true;
            tiles[pos.x + pos.y * size.x].uuid = uuid;

            let bc = Box::new(Goblin::new(pos));
            entities.insert(uuid, bc);
            uuid += 1;
        }  

        Room {
            size,
            init_pos : Vec2::new(1,1),
            tiles,
            entrances : Vec::new(),
            entities,
            corpses : HashMap::new()
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
        let exit_tile = TileType::Exit{
            node_id : neighbor_id,
            exiting_direction : direction
        };

        let size = self.size;
        let mut position : Vec2<usize>;
        // I think I can shrink this by doing NORTH || SOUTH and using apply direction
        match direction {
            Direction::North => {
                let rng = rand::thread_rng().gen_range(1, size.x-1);
                position = Vec2::new(rng, 0);
                self.get_tile_mut(position).id = exit_tile;
                self.entrances.push(Entrance {
                    location : Vec2::new(position.x, position.y+1),
                    direction : direction
                });
            },
            Direction::East => {
                let rng = rand::thread_rng().gen_range(1, size.y-1);
                position = Vec2::new(size.x-1, rng);
                self.get_tile_mut(position).id = exit_tile;
                self.entrances.push(Entrance {
                    location : Vec2::new(position.x-1, position.y),
                    direction : direction
                });
            },
            Direction::South => {
                let rng = rand::thread_rng().gen_range(1, size.x-1);
                position = Vec2::new(rng, size.y-1);
                self.get_tile_mut(position).id = exit_tile;
                self.entrances.push(Entrance {
                    location : Vec2::new(position.x, position.y-1),
                    direction : direction
                });
            },
            Direction::West => {
                let rng = rand::thread_rng().gen_range(1, size.y-1);
                position = Vec2::new(0, rng);
                self.get_tile_mut(position).id = exit_tile;
                self.entrances.push(Entrance {
                    location : Vec2::new(position.x+1, position.y),
                    direction : direction
                });
            }
        }
    }

    pub fn tiles(&self) -> &Vec<Tile> {
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
            if let Some(Tile) = result {
                return !Tile.id.collidable();
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
            else {
                let curr_pos = *m.position();
                let mut res : Option<Attack>;
                {
                    let m_mut : &mut Entity = m.borrow_mut();
                    res = m_mut.update(player, &self.tiles, self.size); 
                }
                if let Some(attack) = res {
                    player.receive_attack(&attack);
                }
                
                // Do this either way. In case I want move+attack action?
                let new_pos = *m.position();
                self.tiles[curr_pos.x + curr_pos.y * self.size.x].occupied = false;
                self.tiles[new_pos.x + new_pos.y * self.size.x].occupied = true;
                self.tiles[new_pos.x + new_pos.y * self.size.x].uuid = *uuid;
                
            }
        }
        
        for uuid in &rem {
            let pos = *self.entities.get(uuid).unwrap().position();
            self.corpses.insert(*uuid, Corpse::new(pos));

            self.get_tile_mut(pos).occupied = false;
            self.get_tile_mut(pos).corpses.push(*uuid);
            self.entities.remove(uuid);
        }
    }

    pub fn get_entities(&self) -> &EntityMap {
        &self.entities
    }

    pub fn get_corpses(&self) -> &CorpseMap {
        &self.corpses
    }

    pub fn get_mut_entities(&mut self) -> &mut EntityMap {
        &mut self.entities
    }

    pub fn handle_player_input( &mut self, 
                                player : &mut Player,
                                new_pos : Vec2<usize>,
                                log : &mut Log) -> TileType
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

        self.get_tile_type(new_pos)
    }

    fn get_tile(&mut self, loc : Vec2<usize>) -> &Tile {
        &self.tiles[loc.x + loc.y * self.size.x]
    }

    fn get_tile_mut(&mut self, loc : Vec2<usize>) -> &mut Tile {
        &mut self.tiles[loc.x + loc.y * self.size.x]
    }

    fn get_tile_type(&self, loc : Vec2<usize>) -> TileType {
        self.tiles[loc.x + loc.y * self.size.x].id
    }
}