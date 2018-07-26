use utils::Graph;
use utils::Vec2;
use entity::{EntityMap};
use player::Player;
use log::Log;
use world::Room;

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