extern crate chrono;

pub mod math;
pub mod player;
pub mod rogue_view;
pub mod world;
pub mod goblin;
pub mod camera;
pub mod attack;
pub mod stats;
pub mod display;
pub mod log;
pub mod input;

use std::collections::HashMap;
use math::Vec2;
use player::Player;
use world::World;
use camera::Camera;
use goblin::Goblin;
use attack::*;
use display::Drawable;
use log::Log;
use input::{Input};

pub struct GameOptions {
    width : usize,
    height : usize,
    player_name : String,
    player_class : player::Class
}

impl GameOptions {
    pub fn new( width : usize, 
                height : usize, 
                player_name : String, 
                player_class : player::Class) -> GameOptions 
    {
        GameOptions {
            width,
            height,
            player_name,
            player_class
        }
    }
}

// Create Entity concept
pub trait Entity : Attackable + Drawable {}
impl<T> Entity for T where T: Attackable + Drawable {}

pub struct Game {
    player : Player,
    world : World,
    camera : Camera,
    step : bool,
    log : Log,
    uuid : u32,
    entities : HashMap<u32, Box<Entity>>
}

impl Game {
    pub fn new(options : GameOptions) -> Self {
        let player = player::Player::new(
                        options.player_name, 
                        Vec2::new(0,0)
                     );

        let mut uuid = 0;
        let mut m  : HashMap<u32, Box<Entity>> = HashMap::new();
        for i in 0..5 {
            let bc = Box::new(Goblin::new(Vec2::new(5+i,5+i)));
            m.insert(uuid, bc);
            uuid += 1;
        }

        Game {
            player,
            world : World::new(options.width, options.height),
            camera : Camera::new(),
            step : false,
            log : Log::new(20),
            uuid,
            entities : m
        }
    }

    fn step(&mut self) {
        let mut rem = Vec::new();

        for (uuid, m) in &mut self.entities {
            if !m.alive() {
                rem.push(*uuid);
            }
        }
        
        for uuid in &rem {
            println!("{}",uuid );
            self.entities.remove(uuid);
        }
    }

    pub fn handle_input(&mut self, input : &Input) {
        match input {
            Input::Right => self.process_move(1, 0),
            Input::Left => self.process_move(-1, 0),
            Input::Up => self.process_move(0, -1),
            Input::Down => self.process_move(0, 1),
            Input::Key(key) => self.process_char(*key),
            _ => {}
        }

        // Update the camera
        let pos = self.player.position();
        self.camera.move_camera(pos.x, pos.y, self.world.width(), self.world.height());

        // Player moved/attacked so update world
        if self.step {
            self.step();
            self.step = false;
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn entities(&self) -> &HashMap<u32, Box<Entity>> {
        &self.entities
    }

    pub fn get_log_messages(&self, msg_count : usize) -> &[String] {
        self.log.last_n_messages(msg_count)
    }

    pub fn active_loot(&self) -> String {
        String::from("Hello!")
    }

    fn process_move(&mut self, x_dir : i32, y_dir : i32) {
        let mut lcl_x = x_dir;
        let mut lcl_y = y_dir;
        let pos = self.player.position();
        if (pos.x as i32 + x_dir) < 0 {
            lcl_x = 0;
        }

        if (pos.y as i32 + y_dir) < 0 {
            lcl_y = 0;
        }

        let new_pos = Vec2::new((pos.x as i32 + lcl_x) as usize, 
                                (pos.y as i32 + lcl_y) as usize);
        
        let mut blocked = false;
        for (uuid, mut m) in &mut self.entities {
            if m.collision(new_pos) {
                let attack = self.player.send_attack();
                let result = m.receive_attack(&attack);
                self.log.log_combat(&self.player, &result);
                if result.target_alive {
                    blocked = true;
                }
            }
        }

        if  !blocked &&
            new_pos.x < self.world.width() && 
            new_pos.y < self.world.height()
        {
            self.player.move_player(lcl_x, lcl_y);
        }

        self.step = true;
    }

    fn process_char(&self, key : char) {
        println!("{}",key);
    }

}