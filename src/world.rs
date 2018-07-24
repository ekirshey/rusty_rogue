use super::graph::*;

#[derive(Copy, Clone)]
pub struct Cell {
    pub x : u32
}

impl Cell {
    fn new() -> Cell {
        Cell {
            x : 0
        }
    }
}

pub struct World {
    width : usize,
    height : usize,
    tiles : Vec<Cell>
}

impl World {
    pub fn new(width : usize, height : usize) -> World {
        let c = Cell::new();
        World {
            width,
            height,
            tiles : vec![c; width*height]
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn tiles(&self) -> &Vec<Cell> {
        &self.tiles
    }
}