extern crate rand;

use utils::Graph;
use utils::Vec2;
use entity::{EntityMap};
use player::Player;
use log::Log;
use world::Room;
use world::Direction;
use world::CellType;
use world::DungeonBuilder;

use self::rand::prelude::*;

// RNG properties
pub struct DungeonProperties {
    min_grid_size : Vec2<usize>,
    max_grid_size : Vec2<usize>,
    min_room_size : Vec2<usize>,
    max_room_size : Vec2<usize>,
    min_depth : usize,
    max_depth : usize,
    min_rooms : usize,
    max_rooms : usize
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

        let start = Vec2::new( rand::thread_rng().gen_range(0, 10),
                            rand::thread_rng().gen_range(0, 10));

        let mut floors = Vec::new();
        floors.push(Graph::new());
        {
            let mut grid = DungeonBuilder::new(start, Vec2::new(10,10), 10, &mut floors[0]);
            grid.build_floor();
        }

        Dungeon {
            depth,
            active_room : 0,
            active_floor : 0,
            entrance_floor : 0,
            entrance_room : 0,
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

    pub fn active_room_id(&self) -> usize {
        self.active_room
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
        let cell_type = self.get_mut_room(floorid, roomid)
                            .handle_player_input(player, new_pos, log);
        

        if let CellType::Exit{node_id, exiting_direction} = cell_type {
            let mut entering_direction = exiting_direction;
            entering_direction.invert();
            
            self.active_room = node_id;
            let new_pos = self.get_room(floorid, node_id).entering_position(entering_direction);

            player.move_player(new_pos);
        }
    }
}