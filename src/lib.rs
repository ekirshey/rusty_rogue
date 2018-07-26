extern crate chrono;

mod utils;
mod entity;
mod world;

pub mod player;
pub mod rogue_view;
pub mod goblin;
pub mod log;
pub mod input;

use utils::Vec2;
use entity::StatBlock;
use entity::EntityMap;
use world::World;
use input::{Input, MouseEvent, MouseButton};

// Change at some point?
pub use player::Player;
pub use goblin::Goblin;
pub use log::Log;

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

pub struct Game {
    player : Player,
    world : World,
    viewport : Vec2<usize>,
    step : bool,
    log : Log
}

impl Game {
    pub fn new(options : GameOptions) -> Self {
        let world = World::new();
        let player = player::Player::new(
                        options.player_name, 
                        world.starting_position()
                     );

        Game {
            player,
            world,
            viewport : Vec2::new(60, 20),
            step : false,
            log : Log::new(20)
        }
    }

    fn step(&mut self) {
        self.world.step(&mut self.player);
    }

    pub fn handle_input(&mut self, input : &Input) {    
        match input {
            Input::Right => self.process_move(1, 0),
            Input::Left => self.process_move(-1, 0),
            Input::Up => self.process_move(0, -1),
            Input::Down => self.process_move(0, 1),
            Input::Key(key) => self.process_char(*key),
            Input::Mouse{offset, position, event} => self.process_mouse(*position, event),
            _ => {}
        }

        // Player moved/attacked so update world
        if self.step {  // Only certain input events trigger a step
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

    pub fn get_entities(&self) -> Option<&EntityMap> {
        self.world.get_entities()
    }

    pub fn viewport_width(&self) -> usize {
        self.viewport.x
    }

    pub fn viewport_height(&self) -> usize {
        self.viewport.y
    }

    pub fn get_log_messages(&self, msg_count : usize) -> &[String] {
        self.log.last_n_messages(msg_count)
    }

    pub fn active_loot(&self) -> String {
        String::from("Hello!")
    }

    // Target Functions
    // I can't seem to return a &Box<Entity> from a function
    // Possibly because of lifetimes? Either way I'm making wrappers
    // that take a uuid
    pub fn active_target(&self) -> bool {
        if let Some(uuid) = self.player.target() {
            return true;
        }
        false
    }

    pub fn target_name(&self) -> Option<&str> {
        if let Some(uuid) = self.player.target() {
            let result = self.get_entities();
            if let Some(entities) = result {
                return Some(entities[&uuid].name());
            }
        }
        None
    }

    pub fn target_current_stats(&self) -> Option<&StatBlock> {
        if let Some(uuid) = self.player.target() {
            let result = self.get_entities();
            if let Some(entities) = result {
                return Some(entities[&uuid].current_stats());
            }
        }
        None
    }

    pub fn target_base_stats(&self) -> Option<&StatBlock> {
        if let Some(uuid) = self.player.target() {
            let result = self.get_entities();
            if let Some(entities) = result {
                return Some(entities[&uuid].base_stats());
            }
        }
        None
    }
    
    /////////////////////////////////////////////////

    fn process_move(&mut self, x_dir : i32, y_dir : i32) {
        let mut lcl_x = x_dir;
        let mut lcl_y = y_dir;
        let new_pos : Vec2<usize>;

        {
            let pos = self.player.position();
            if (pos.x as i32 + x_dir) < 0 {
                lcl_x = 0;
            }

            if (pos.y as i32 + y_dir) < 0 {
                lcl_y = 0;
            }

            new_pos = Vec2::new((pos.x as i32 + lcl_x) as usize, 
                                (pos.y as i32 + lcl_y) as usize);
        }

        self.world.handle_player_input(&mut self.player, new_pos, &mut self.log);

        self.step = true;
    }

    fn process_char(&self, key : char) {
        println!("{}",key);
    }

    fn process_mouse(&mut self, position : Vec2<usize>, event : &input::MouseEvent)  {
        if let MouseEvent::Press(MouseButton::Left) = event {
            let result = self.world.get_entities();
            if let Some(entities) = result {
                for (uuid, m) in entities {
                    if m.collision(position) {
                        self.player.set_target(*uuid);
                    }
                }
            }
        }
    }

}