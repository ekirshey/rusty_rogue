use utils::Vec2;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    pub fn get_direction(val : usize) -> Direction {
        match val {
            1 => Direction::North,
            2 => Direction::East,
            3 => Direction::South,
            4 => Direction::West,
            _ => Direction::North   // by default north
        }
    }

    pub fn invert(&mut self) {
         match *self {
            Direction::North => *self = Direction::South,
            Direction::East => *self = Direction::West,
            Direction::South => *self = Direction::North, 
            Direction::West => *self = Direction::East,
        }        
    }

    pub fn rotate_cw(&mut self) {
        match *self {
            Direction::North => *self = Direction::East,
            Direction::East => *self = Direction::South,
            Direction::South => *self = Direction::West, 
            Direction::West => *self = Direction::North,
        }        
    }

    pub fn rotate_ccw(&mut self) {
        match *self {
            Direction::North => *self = Direction::West,
            Direction::West => *self = Direction::South,
            Direction::South => *self = Direction::East, 
            Direction::East => *self = Direction::North,          
        }        
    }

    pub fn try_apply(&self, location : Vec2<usize>)  -> Option<Vec2<usize>> {
        match &self {
            Direction::North => {
                if location.y >= 1 {
                    return Some(Vec2::new(location.x, location.y - 1));
                }
            },
            Direction::East => {
                return Some(Vec2::new(location.x + 1, location.y));
            },
            Direction::South => {
                return Some(Vec2::new(location.x, location.y + 1));
            },
            Direction::West => {
                if location.x >= 1 {
                    return Some(Vec2::new(location.x -1, location.y));
                }
            }
        }

        None
    }
}