use super::utils::graph::Graph;
use super::utils::math::Vec2;
use super::Entity;

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
    tiles : Vec<Cell>
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

        Room {
            size,
            tiles,
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
}

pub struct Dungeon {
    depth : usize,
    active_room : usize,
    active_floor : usize,
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
            floors
        }
    }

    pub fn active_room(&self) -> &Room {
        let result = self.floors[self.active_floor].get(self.active_room);
        if let Some(node) = result {
            return &node.data;
        }
        else {
            panic!("World active node is invalid!");
        }
    }

}