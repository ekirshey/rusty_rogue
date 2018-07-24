extern crate cursive;

use super::player;
use super::{ Game, GameOptions, Input};
use super::world::Cell;
use super::utils::math;
use super::input;
use super::attack::Attackable;
use super::Entity;

use self::cursive::Printer;
use self::cursive::theme::{Color, ColorStyle, Effect};
use self::cursive::vec::Vec2;
use self::cursive::direction::Direction;
use self::cursive::event::{Event, MouseEvent, MouseButton, EventResult, Key};

pub struct RogueView {
    game : Game,
    width : usize,
    height : usize,
}

impl RogueView {
    pub fn new(size : Vec2, name : String, class : player::Class) -> RogueView {
        let options = GameOptions::new(60, 30, name, class);

        RogueView {
            game : Game::new(options),
            width : size.x,
            height : size.y
        }
    }

    pub fn active_loot(&self) -> String {
        self.game.active_loot()
    }

    fn draw_player_info(&self, start : Vec2, printer: &Printer) {
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

    fn draw_target_info(&self, start : Vec2, printer: &Printer) {
        let camera = self.game.camera();
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
        let camera = self.game.camera();

        printer.print_vline( (camera.width, 0), self.width, "│");

        printer.print_hline((camera.width+1,self.height/2), self.width-camera.width, "─");

        printer.print((camera.width+1, 0), "Player:");

        printer.print((camera.width+1, self.height/2+1), "Target:");

        self.draw_player_info(Vec2::new(camera.width+1, 1), printer);
        if self.game.active_target() {
            self.draw_target_info(Vec2::new(camera.width+1, self.height/2+2), printer);
        }
    }

    fn draw_log(&self, printer: &Printer) {
        let camera = self.game.camera();

        // Draw Borders
        printer.print_hline((0,camera.height), camera.width, "─");

        let log_size = self.height - camera.height-1;

        let msgs = self.game.get_log_messages(log_size);
        for (i,msg) in msgs.iter().enumerate() {
            printer.print((0, camera.height+1+i), msg);
        }
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
                let mut bg = Color::Rgb(display.bg.x,display.bg.y,display.bg.z);
                if let Some(target) = self.game.target {
                    if target == *uuid {
                        bg = Color::Rgb(50,50,50);
                    }
                }
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

        self.draw_log(printer);
        self.draw_info(printer);

    }

    fn take_focus(&mut self, _: Direction) -> bool {
        true
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        //else if let Event::Char(key) = event {
        //    input = Input::Key(key);
        //}
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

            input = Input::Mouse{
                offset : math::Vec2::new(offset.x,offset.y),
                position : math::Vec2::new(position.x,position.y),
                event : new_event,
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