use world::Direction;

#[derive(Debug)]
pub enum CellType {
    Wall,
    Granite,
    Exit {
        node_id : usize,
        exiting_direction : Direction
    },
}

impl CellType {
    pub fn value(&self) -> char {
        match *self {
            CellType::Wall => '#',
            CellType::Granite => '.',
            CellType::Exit{node_id, exiting_direction} => ' ',
        }
    }

    pub fn collidable(&self) -> bool {
        match *self {
            CellType::Wall => true,
            CellType::Granite => false,
            CellType::Exit{node_id, exiting_direction} => false,
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