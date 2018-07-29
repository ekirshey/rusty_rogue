extern crate cursive;

use super::player;
use super::{ Game, GameOptions, Input};
use utils;
use super::input;
use world::{WorldNode, Dungeon};

use self::cursive::Printer;
use self::cursive::theme::{Color, ColorStyle, Effect};
use self::cursive::vec;
use self::cursive::direction::Direction;
use self::cursive::event::{Event, MouseEvent, MouseButton, EventResult, Key};

pub struct RogueView {
    game : Game,
    width : usize,
    height : usize,
    offset : utils::Vec2<usize>
}

impl RogueView {
    pub fn new(size : vec::Vec2, name : String, class : player::Class) -> RogueView {
        let options = GameOptions::new(60, 30, name, class);

        let mut rogueview = RogueView {
            game : Game::new(options),
            width : size.x,
            height : size.y,
            offset : utils::Vec2::new(0,0)
        };
        rogueview.update_room_offset();

        rogueview
    }

    pub fn active_loot(&self) -> String {
        self.game.active_loot()
    }

    fn draw_player_info(&self, start : vec::Vec2, printer: &Printer) {
        let player = self.game.player();
        let curr_stats = player.current_stats();
        let base_stats = player.base_stats();

        let green = Color::Rgb(0,255,0);
        let bg = Color::Rgb(95,95,95);
        let mut x = start.x;
        let mut y = start.y;
        printer.with_effect( Effect::Bold,
            |printer| printer.print((x,y), "Name:")
        );
        x += 6;
        printer.with_color(
            ColorStyle::new(green, bg),
            |printer| printer.print((x,y), player.name())
        );
        y += 1;
        x = start.x;

        printer.with_effect( Effect::Bold,
            |printer| printer.print((x,y), "Health: ")
        );

        x += 8;

        let curr_health = curr_stats.health.to_string() + "/";
        let base_health = base_stats.health.to_string();
        printer.with_color(
            ColorStyle::new(green, bg),
            |printer| printer.print((x,y), curr_health.as_ref())
        );

        x += curr_health.len();
        printer.with_color(
            ColorStyle::new(green, bg),
            |printer| printer.print((x,y), base_health.as_ref())
        );
    }


    fn draw_target_info(&self, start : vec::Vec2, printer: &Printer) {
        let curr_stats = self.game.target_current_stats().unwrap();
        let base_stats = self.game.target_base_stats().unwrap();
        let name = self.game.target_name().unwrap();

        let green = Color::Rgb(0,255,0);
        let bg = Color::Rgb(95,95,95);
        let mut x = start.x;
        let mut y = start.y;
        printer.with_effect( Effect::Bold,
            |printer| printer.print((x,y), "Name:")
        );
        x += 6;
        printer.with_color(
            ColorStyle::new(green, bg),
            |printer| printer.print((x,y), name)
        );
        y += 1;
        x = start.x;

        printer.with_effect( Effect::Bold,
            |printer| printer.print((x,y), "Health: ")
        );

        x += 8;

        let curr_health = curr_stats.health.to_string() + "/";
        let base_health = base_stats.health.to_string();
        printer.with_color(
            ColorStyle::new(green, bg),
            |printer| printer.print((x,y), curr_health.as_ref())
        );

        x += curr_health.len();
        printer.with_color(
            ColorStyle::new(green, bg),
            |printer| printer.print((x,y), base_health.as_ref())
        );
    }


    fn draw_info(&self, printer: &Printer) {
        let vp_width = self.game.viewport_width();

        printer.print_vline( (vp_width, 0), self.width, "│");

        printer.print_hline((vp_width+1,self.height/2), self.width-vp_width, "─");

        printer.print((vp_width+1, 0), "Player:");

        printer.print((vp_width+1, self.height/2+1), "Target:");

        self.draw_player_info(vec::Vec2::new(vp_width+1, 1), printer);
        if self.game.active_target() {
            self.draw_target_info(vec::Vec2::new(vp_width+1, self.height/2+2), printer);
        }

    }

    fn draw_log(&self, printer: &Printer) {
        let vp_width = self.game.viewport_width();
        let vp_height = self.game.viewport_height();

        // Draw Borders
        printer.print_hline((0,vp_height), vp_width, "─");

        let log_size = self.height - vp_height-1;

        let msgs = self.game.get_log_messages(log_size);
        for (i,msg) in msgs.iter().enumerate() {
            printer.print((0, vp_height+1+i), msg);
        }
    }

    fn draw_dungeon_room(&self, dungeon : &Dungeon, printer: &Printer) {
        let player = self.game.player();
        let vp_width = self.game.viewport_width();
        let vp_height = self.game.viewport_height();

        let room = dungeon.active_room();

        printer.print((0,0), &dungeon.active_room_id().to_string());

        for ( i, cell) in room.tiles().iter().enumerate() {
            let x = i % room.width();
            let y = i / room.width();

            let display = cell.id.value();
            let symbol = display.icon.to_string();
            let fg = Color::Rgb( display.fg.x, display.fg.y, display.fg.z );
            let bg = Color::Rgb( display.bg.x, display.bg.y, display.bg.z );

            printer.with_color(
                ColorStyle::new(fg, bg),
                |printer| printer.print(
                                (self.offset.x + x, self.offset.y + y), &symbol),
            );
        } 

        // draw entities
        let result = self.game.get_entities();
        if let Some(entities) = result {
            for (uuid, e) in entities.iter() {
                let display = e.draw();
                let pos = display.position;

                let symbol = display.icon.to_string();
                let fg = Color::Rgb( display.fg.x, display.fg.y, display.fg.z );
                let mut bg = Color::Rgb( display.bg.x, display.bg.y, display.bg.z );
                let result = self.game.player().target();
                if let Some(target) = result {
                    if target == *uuid {
                        bg = Color::Rgb(50,50,50);
                    }
                }
                printer.with_color(
                    ColorStyle::new(fg, bg),
                    |printer| printer.print(
                                    (self.offset.x + pos.x, self.offset.y + pos.y), &symbol),
                );
                
            }
        }

        // Draw Player
        let pos = player.position();
        let pos_x = self.offset.x + pos.x;
        let pos_y = self.offset.y + pos.y;
        printer.print((pos_x, pos_y), "@");
    }

    fn update_room_offset(&mut self) {
        // Calculate offset
        let world = self.game.world();

        match world.active_node() {
            WorldNode::DungeonNode(ref dungeon) => {
                        let room = dungeon.active_room();
                        
                        let vp_width = self.game.viewport_width();
                        let vp_height = self.game.viewport_height();

                        let x_offset = vp_width/2 - room.width()/2;
                        let y_offset = vp_height/2 - room.height()/2;
                        self.offset = utils::Vec2::new(x_offset, y_offset);
            }
            _ => println!("No draw for this not type yet"),
        }   
    }
}

impl cursive::view::View for RogueView {
    fn draw(&self, printer: &Printer) {
        let world = self.game.world();

        match world.active_node() {
            WorldNode::DungeonNode(ref dungeon) => self.draw_dungeon_room(dungeon, printer),
            _ => println!("No draw for this not type yet"),
        }   

        self.draw_log(printer);
        self.draw_info(printer);

    }

    fn take_focus(&mut self, _: Direction) -> bool {
        true
    }

    fn on_event(&mut self, event: Event) -> EventResult {   
        let mut input : input::Input = Input::Unknown;
        if let Event::Key(key) = event {     
            input = match key {
                Key::Right => Input::Right,
                Key::Left => Input::Left,
                Key::Up => Input::Up,
                Key::Down => Input::Down,
                _ => Input::Unknown
            };
        }
        else if let Event::Mouse{offset ,position, event} = event {
            let new_event = match event {
                MouseEvent::Press(button) => input::MouseEvent::Press(
                    match button {
                        MouseButton::Left => input::MouseButton::Left,
                        MouseButton::Right => input::MouseButton::Right,
                        _ => input::MouseButton::Unknown
                    }
                ),
                MouseEvent::Release(button) => input::MouseEvent::Release(
                    match button {
                        MouseButton::Left => input::MouseButton::Left,
                        MouseButton::Right => input::MouseButton::Right,
                        _ => input::MouseButton::Unknown
                    }
                ),
                MouseEvent::Hold(button) => input::MouseEvent::Hold(
                    match button {
                        MouseButton::Left => input::MouseButton::Left,
                        MouseButton::Right => input::MouseButton::Right,
                        _ => input::MouseButton::Unknown
                    }
                ),
                MouseEvent::WheelUp => input::MouseEvent::WheelUp,
                MouseEvent::WheelDown => input::MouseEvent::WheelUp,
            };

            let mut mouse_position = utils::Vec2::new(position.x, position.y);
            if position.x >= self.offset.x {
                mouse_position.x -= self.offset.x;
            }
            if position.y >= self.offset.y {
                mouse_position.y -= self.offset.y;
            }
            input = Input::Mouse{
                offset : utils::Vec2::new(offset.x,offset.y),
                position : mouse_position,
                event : new_event,
            };
        }

        if input != Input::Unknown {
            self.game.handle_input(&input);
            self.update_room_offset();
            return EventResult::Consumed(None);
        }

        EventResult::Ignored
    }

    fn required_size(&mut self, _: vec::Vec2) -> vec::Vec2 {
        vec::Vec2 {
            x : self.width,
            y : self.height
        }
    }
}