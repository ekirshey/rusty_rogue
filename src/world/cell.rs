pub enum CellType {
    Wall,
    Granite,
    Exit,
}

impl CellType {
    pub fn value(&self) -> char {
        match *self {
            CellType::Wall => '#',
            CellType::Granite => '.',
            CellType::Exit => ' ',
        }
    }

    pub fn collidable(&self) -> bool {
        match *self {
            CellType::Wall => true,
            CellType::Granite => false,
            CellType::Exit => false,
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