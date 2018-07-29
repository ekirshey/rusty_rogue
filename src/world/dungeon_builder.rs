extern crate rand;

use utils::Graph;
use utils::Vec2;
use world::Room;
use world::Direction;

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

pub struct DungeonBuilder<'a> {
    size : Vec2<usize>,
    num_rooms : usize,
    room_ct : usize,
    start_cell : Vec2<usize>,
    grid : Vec<GridElement>,
    graph : &'a mut Graph<Room>
}

impl<'a> DungeonBuilder<'a> {
    // Enforce start is within bounds
    pub fn new(start : Vec2<usize>, size : Vec2<usize>, num_rooms : usize, graph : &'a mut Graph<Room>) -> DungeonBuilder {
        let mut grid = Vec::new();
        for i in 0..size.x*size.y {
            let x = i % size.x;
            let y = i / size.y;

            grid.push(GridElement::new(Vec2::new(x,y)));
        }

        DungeonBuilder {
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

    fn get_rand_direction(&self) -> Direction {
        Direction::get_direction(rand::thread_rng().gen_range(0, 4))
    }

    fn build_room(&mut self, location : Vec2<usize>) {

        if self.room_ct == self.num_rooms ||
           location.x >= self.size.x || 
           location.y >= self.size.y 
        {
            return;
        }

        if !self.get_cell(location).has_room {
            self.get_cell_mut(location).has_room = true;
            self.room_ct += 1;
        }
        
        let mut direction = self.get_rand_direction();

        let mut stop = false;
        let mut iterations = 0;

        while !stop && iterations < 4 {
            let res = direction.try_apply(location);

            if let Some(next_cell) = res {
                self.build_room(next_cell);
            }

            if self.room_ct == self.num_rooms {
                stop = true;
            }
            iterations += 1;
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
                    if next_cell.x < self.size.x &&
                       next_cell.y < self.size.y &&
                       self.get_cell(next_cell).has_room 
                    {
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

    fn debug_print(&self) {
        let mut ct = 0;
        for i in &self.grid {
            if i.has_room {
                if i.node < 10 {
                    print!("0");
                }
                print!("{} ", i.node);
            }
            else {
                print!("__ ");
            }
            ct += 1;
            if ct >= self.size.x {
                println!("");
                ct = 0;
            }
        }

        println!("start at {:?}",self.start_cell );
        println!("room ct {}", self.room_ct );
    }

    pub fn build_floor(&mut self) {
        let start = self.start_cell;
        self.build_room(start);
        self.populate_graph(start);

        self.debug_print();
    }
}
