use world::Direction;
use utils::Vec3;

pub struct CellDisplay {
    pub fg : Vec3<u8>,
    pub bg : Vec3<u8>,
    pub icon : char
}

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
    pub fn value(&self) -> CellDisplay {
        match *self {
            CellType::Wall => CellDisplay {
                fg : Vec3::new(255, 255, 255),
                bg : Vec3::new(95,95,95),
                icon : '#'
            },
            CellType::Granite => CellDisplay {
                fg : Vec3::new(255, 255, 255),
                bg : Vec3::new(95,95,95),
                icon : '.'
            },
            CellType::Exit{node_id, exiting_direction} => CellDisplay {
                fg : Vec3::new(0, 0, 0),
                bg : Vec3::new(255, 242, 0),
                icon : ' '
            },
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