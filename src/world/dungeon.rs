extern crate rand;

use utils::Graph;
use utils::Vec2;
use entity::{EntityMap};
use player::Player;
use log::Log;
use world::Room;
use world::Direction;
use world::CellType;

use self::rand::prelude::*;

// internal structures for managing room construction
struct GridElement {
    pub has_room : bool,
    pub filled : bool,
    pub node : usize,
    pub left_corner : Vec2<usize>
}

impl GridElement {
    pub fn new(left_corner : Vec2<usize>) -> GridElement {
        GridElement{
            has_room : false,
            filled : false,
            node : 0,
            left_corner
        }
    }
}

struct DungeonGrid<'a> {
    size : Vec2<usize>,
    num_rooms : usize,
    room_ct : usize,
    start_cell : Vec2<usize>,
    grid : Vec<GridElement>,
    graph : &'a mut Graph<Room>
}

impl<'a> DungeonGrid<'a> {
    // Enforce start is within bounds
    pub fn new(start : Vec2<usize>, size : Vec2<usize>, num_rooms : usize, graph : &'a mut Graph<Room>) -> DungeonGrid {
        let mut grid = Vec::new();
        for i in 0..size.x*size.y {
            let x = i % size.x;
            let y = i / size.y;

            grid.push(GridElement::new(Vec2::new(x,y)));
        }

        DungeonGrid {
            size,
            num_rooms : num_rooms,
            room_ct : 0,
            start_cell : start,
            grid,
            graph
        }
    }

    fn get_cell(&self, location : Vec2<usize>) -> &GridElement {
        &self.grid[location.x + location.y * self.size.x]
    }

    fn get_cell_mut(&mut self, location : Vec2<usize>) -> &mut GridElement {
        &mut self.grid[location.x + location.y * self.size.x]
    }

    fn build_room(&mut self, location : Vec2<usize>) {
        if self.room_ct == self.num_rooms ||
           self.get_cell(location).has_room ||
           location.x >= self.size.x || 
           location.y >= self.size.y 
        {
            return;
        }

        self.get_cell_mut(location).has_room = true;
        self.room_ct += 1;

        let mut direction = Direction::get_direction(rand::thread_rng().gen_range(0, 4));

        let mut stop = false;
        while !stop {
            let res = direction.try_apply(location);

            if let Some(next_cell) = res {
                self.build_room(next_cell);

                // continue? need to add a weight
                let rng = rand::thread_rng().gen_range(0, 4);
                if rng <= 1 {
                    stop = true;
                }
            }

            direction.rotate_cw();
        }   
    }

    fn populate_graph(&mut self, location : Vec2<usize>) -> usize {
        let mut id = self.get_cell(location).node;
        // Has a room and the room hasnt been built yet
        if self.get_cell(location).has_room && 
           !self.get_cell(location).filled 
        {
            let mut direction = Direction::North;

            let rng_size = Vec2::new(rand::thread_rng().gen_range(5, 16),
                                      rand::thread_rng().gen_range(5, 16));
            id = self.graph.new_node(Room::new(rng_size));
            
            self.get_cell_mut(location).filled = true;
            self.get_cell_mut(location).node = id;

            let mut stop = false;
            while !stop {
                let res = direction.try_apply(location);
                if let Some(next_cell) = res {
                    if self.get_cell(next_cell).has_room {
                        let neighbor = self.populate_graph(next_cell);
                        self.graph.add_neighbor(id, neighbor).unwrap();
                        
                        {
                            let mut node = self.graph.get_mut(id).unwrap();
                            node.data.add_neighbor( direction, neighbor );
                        }
                    }
                }

                direction.rotate_cw();
                if direction == Direction::North {
                    stop = true;
                }
            }
        }  

        id
    }

    pub fn build_floor(&mut self) {
        let start = self.start_cell;
        self.build_room(start);
        self.populate_graph(start);
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
        let start = Vec2::new( rand::thread_rng().gen_range(0, 10),
                            rand::thread_rng().gen_range(0, 10));

        let mut floors = Vec::new();
        floors.push(Graph::new());
        {
            let mut grid = DungeonGrid::new(start, Vec2::new(10,10), 10, &mut floors[0]);
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
            println!("exiting to {}", node_id );
            let mut entering_direction = exiting_direction;
            entering_direction.invert();
            
            self.active_room = node_id;
            let new_pos = self.get_room(floorid, node_id).entering_position(entering_direction);

            player.move_player(new_pos);
        }
    }
}