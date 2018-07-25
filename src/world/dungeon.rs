use utils::Graph;
use utils::Vec2;
use entity::{EntityMap};
use player::Player;
use log::Log;
use std::collections::HashMap;

// Entities
use goblin::Goblin;

pub enum CellType {
    Wall,
    Granite,
}

impl CellType {
    pub fn value(&self) -> char {
        match *self {
            CellType::Wall => '#',
            CellType::Granite => '.',
        }
    }

    pub fn collidable(&self) -> bool {
        match *self {
            CellType::Wall => true,
            CellType::Granite => false,
        }
    }
}

impl Copy for CellType { }

impl Clone for CellType {
    fn clone(&self) -> CellType {
        *self
    }
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub id : CellType
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            id : CellType::Wall
        }
    }
}

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
        for i in 0..5 {
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

pub struct Dungeon {
    depth : usize,
    active_floor : usize,
    active_room : usize,
    entrance_floor : usize,
    entrance_room : usize,
    floors : Vec<Graph<Room>>
}

impl Dungeon {
    pub fn new(depth : usize) -> Dungeon {
        // Dummy 1 room dungeon
        let mut floors = Vec::new();
        let mut f1 = Graph::new();
        let active_room = f1.new_node(Room::new(Vec2::new(10,10)));
        floors.push(f1);

        Dungeon {
            depth,
            active_room,
            active_floor : 0,
            entrance_floor : 0,
            entrance_room : active_room,
            floors
        }
    }

    // Decide how you want to handle invalid active floor
    // panic for now but maybe just set to some normal value?
    fn get_mut_room(&mut self, floor : usize, room : usize) -> &mut Room{
        let result = self.floors[floor].get_mut(room);
        if let Some(node) = result {
            return &mut node.data;
        }
        else {
            panic!("Invalid floor and room");
        }
    }

    fn get_room(&self, floor : usize, room : usize) -> &Room {
        let result = self.floors[floor].get(room);
        if let Some(node) = result {
            return &node.data;
        }
        else {
            panic!("Invalid floor and room");
        }
    }

    pub fn active_room(&self) -> &Room {
        self.get_room(self.active_floor, self.active_room)
    }

    pub fn starting_position(&self) -> Vec2<usize> {
        let room = self.get_room(self.entrance_floor, self.entrance_room);
        room.initial_position()
    }

    pub fn valid_position(&self, pos : Vec2<usize>) -> bool{
        let room = self.get_room(self.active_floor, self.active_room);
        room.valid_position(pos)
    }

    pub fn get_entities(&self) -> &EntityMap {
        let room = self.get_room(self.active_floor, self.active_room);
        room.get_entities()
    }

    pub fn get_mut_entities(&mut self) -> &mut EntityMap {
        let floorid = self.active_floor;
        let roomid = self.active_room;
        let room = self.get_mut_room(floorid, roomid);
        room.get_mut_entities()
    }

    pub fn step(&mut self, player : &mut Player) {
        let floorid = self.active_floor;
        let roomid = self.active_room;
        let room = self.get_mut_room(floorid, roomid);
        room.step(player);
    }

    pub fn handle_player_input( &mut self, 
                                player : &mut Player,
                                new_pos : Vec2<usize>,
                                log : &mut Log)  
    {
        let floorid = self.active_floor;
        let roomid = self.active_room;
        let room = self.get_mut_room(floorid, roomid);
        room.handle_player_input(player, new_pos, log);
    }
}