extern crate cursive;

use super::player;
use super::{ Game, GameOptions, Input};
use super::world::Cell;
use super::math;
use super::input;

use self::cursive::Printer;
use self::cursive::theme::{BaseColor, Color, ColorStyle};
use self::cursive::vec::Vec2;
use self::cursive::direction::Direction;
use self::cursive::event::{Event, EventResult, Key};

pub struct RogueView {
    game : Game,
    width : usize,
    height : usize,
}

impl RogueView {
    pub fn new(name : String, class : player::Class) -> RogueView {
        let options = GameOptions::new(60, 30, name, class);

        RogueView {
            game : Game::new(options),
            width : 70,
            height : 20
        }
    }

    pub fn active_loot(&self) -> String {
        self.game.active_loot()
    }
}

impl cursive::view::View for RogueView {
    fn draw(&self, printer: &Printer) {
        let world = self.game.world();
        let player = self.game.player();
        let camera = self.game.camera();
        // Only iterate over tiles in camera
        for ( i, cell) in world.tiles().iter().enumerate() {
            let x = i % world.width();
            let y = i / world.width();

            if camera.point_intersects(x, y) {
                let symbol = match cell {
                    Cell { x: 0, .. } => ".",
                    Cell { x: 1, .. } => "+",
                    _ => "0"
                };

                printer.print((x - camera.x,y - camera.y), symbol);
            }
        }

        // draw entities
        for (uuid, e) in self.game.entities().iter() {
            let display = e.draw();
            let pos = display.position;
            if camera.point_intersects(pos.x as usize, pos.y as usize) {
                let symbol = display.icon.to_string();
                let fg = Color::Rgb(display.fg.x,display.fg.y,display.fg.z);
                let bg = Color::Rgb(display.bg.x,display.bg.y,display.bg.z);
                printer.with_color(
                    ColorStyle::new(fg, bg),
                    |printer| printer.print((pos.x - camera.x, pos.y - camera.y), &symbol),
                );
            }
        }

        let pos = player.position();
        let pos_x = pos.x - camera.x;
        let pos_y = pos.y - camera.y;
        printer.print((pos_x, pos_y), "@");

        // Draw Borders
        printer.print_hline((0,camera.height), camera.width, "─");
        printer.print_vline( (camera.width, 0), self.width, "│");

        let log_size = self.height - camera.height-1;
        let msgs = self.game.get_log_messages(log_size);
        for (i,msg) in msgs.iter().enumerate() {
            printer.print((0, camera.height+1+i), msg);
        }

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
        //else if let Event::Char(key) = event {
        //    input = Input::Key(key);
        //}
        else if let Event::Mouse{offset ,position, event} = event {
            println!("{:?} {:?} ",  position, event );
            input = Input::Mouse{
                offset : math::Vec2::new(offset.x,offset.y),
                position : math::Vec2::new(position.x,position.y),
                event : input::MouseEvent::WheelUp
            };
        }

        if input != Input::Unknown {
            self.game.handle_input(&input);
            return EventResult::Consumed(None);
        }

        EventResult::Ignored
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        Vec2 {
            x : self.width,
            y : self.height
        }
    }
}