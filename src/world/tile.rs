use world::Direction;
use utils::Vec3;

// I need to rethink this tile structure
// I think I put too much into tiletype instead of tile
pub struct TileDisplay {
    pub fg : Vec3<u8>,
    pub bg : Vec3<u8>,
    pub icon : char
}

#[derive(Debug)]
pub enum TileType {
    Wall,
    Granite,
    Exit {
        node_id : usize,
        exiting_direction : Direction
    },
}

impl TileType {
    // Maybe move this out into some sort of "colorizer" object
    // that takes in the tile type and the dungeon "theme" to 
    // determine what the appropraite color should be
    pub fn value(&self) -> TileDisplay {
        match *self {
            TileType::Wall => TileDisplay {
                fg : Vec3::new(255, 255, 255),
                bg : Vec3::new(95,95,95),
                icon : '#'
            },
            TileType::Granite => TileDisplay {
                fg : Vec3::new(255, 255, 255),
                bg : Vec3::new(95,95,95),
                icon : '.'
            },
            TileType::Exit{node_id, exiting_direction} => TileDisplay {
                fg : Vec3::new(0, 0, 0),
                bg : Vec3::new(255, 242, 0),
                icon : ' '
            },
        }
    }

    pub fn collidable(&self) -> bool {
        match *self {
            TileType::Wall => true,
            TileType::Granite => false,
            TileType::Exit{node_id, exiting_direction} => false,
        }
    }
}

impl Copy for TileType { }

impl Clone for TileType {
    fn clone(&self) -> TileType {
        *self
    }
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub id : TileType,
    pub occupied : bool,
    pub uuid : usize
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            id : TileType::Wall,
            occupied : false,
            uuid : 0
        }
    }
}