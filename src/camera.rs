pub struct Camera {
    pub x : usize,
    pub y : usize,
    pub width : usize,
    pub height : usize
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            x : 0,
            y : 0,
            width : 50,
            height : 15
        }
    }
}
impl Camera {
    pub fn new() -> Camera {
        Camera::default()
    }

    pub fn point_intersects(&self, x : usize, y : usize) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height 
    }

    pub fn move_camera(&mut self, x : usize, y : usize, x_bound : usize, y_bound : usize) {
        let cam_x_max = ((self.x + self.width) as i32 - 5) as usize;
        let cam_x_min = self.x + 5;
        if x > cam_x_max && (self.x + self.width) < x_bound {
            self.x += 1;
        }

        if x < cam_x_min && self.x > 0 {
            self.x -= 1;
        }

        let cam_y_max = ((self.y + self.height) as i32 -2) as usize;
        let cam_y_min = (self.y + 2);
        if y > cam_y_max && (self.y + self.height) < y_bound {
            self.y += 1;
        }

        if y < cam_y_min && self.y > 0 {
            self.y -= 1;
        }        
    }
}