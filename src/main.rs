extern crate cursive;
extern crate rusty_rogue;

use std::rc::Rc;
use rusty_rogue::player;
use rusty_rogue::rogue_view;

use cursive::Cursive;
use cursive::event::EventResult;
use cursive::view::SizeConstraint;
use cursive::views::{Dialog, TextView, Button, OnEventView, BoxView,
                     LinearLayout, SelectView, EditView};
use cursive::traits::*;

fn main() {
    let mut siv = Cursive::default();
    
    siv.load_theme_file("F:/rust_projects/rusty_rogue/src/assets/theme.toml").unwrap();

    siv.add_layer(
        Dialog::new()
            .title("Rusty Rogue")
            .padding((2, 2, 1, 1))
            .content(
                LinearLayout::vertical()
                    .child(Button::new_raw("  New game   ", show_options))
                    .child(Button::new_raw("    Exit     ", |s| s.quit())),
            ),
    );

    siv.run();
}

fn show_options(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Create Character")
            .content(
                LinearLayout::vertical()
                    .child(LinearLayout::horizontal()
                        .child(TextView::new("Name:"))
                        .child(EditView::new()
                                    .with_id("name")
                                    .fixed_width(12)))
                    .child(SelectView::<player::Class>::new()
                        .item(
                            "Warrior",                         
                            player::Class::Warrior
                        )
                        .item(
                            "Mage",
                            player::Class::Mage
                        )
                        .item(
                            "Rogue",
                            player::Class::Rogue
                        )
                        .with_id("select")
                    )           
            )
            .button("Submit", |s| {
                let name = s.call_on_id("name", | view: &mut EditView| {
                    view.get_content()
                }).unwrap();

                let class = s.call_on_id("select", | view: &mut SelectView<player::Class>| {
                    view.selection()
                }).unwrap();

                if name.len() > 0 {
                    s.pop_layer();
                    let name = Rc::try_unwrap(name).unwrap();
                    let class = Rc::try_unwrap(class).unwrap();
                    new_game(s, name, class);
                }
            })
            .dismiss_button("Back"),
    );
}

fn new_game(siv: &mut Cursive, name : String, class : player::Class) {
    let bv = BoxView::with_full_screen(
                rogue_view::RogueView::new(siv.screen_size(), name, class).with_id("rogue"));
    let bv = OnEventView::new(bv)
        .on_pre_event('l', |s| {
            let inventory = s.call_on_id("rogue", | view: &mut rogue_view::RogueView| {
                view.active_loot()
            }).unwrap();
            // Inventory stuff

            s.add_layer(
                Dialog::new()
                    .title("Loot")
                    .button("Loot All", |s| {
                        s.pop_layer();
                    })
                    .button("Cancel", |s| {
                        s.pop_layer();
                    })
            );
            Some(EventResult::Consumed(None))
        });
      
    siv.add_fullscreen_layer(
        bv
    );
}