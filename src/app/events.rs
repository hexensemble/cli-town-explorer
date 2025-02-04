use crossterm::event::{self, Event, KeyCode};
use std::io;

pub struct EventHander {}

impl EventHander {
    pub fn update(
        main: &mut crate::ui::widgets::MainWidget,
        menu: &mut crate::ui::widgets::MenuWidget,
        state: &mut super::state::State,
    ) -> io::Result<bool> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up => menu.previous(),
                KeyCode::Down => menu.next(),
                KeyCode::Enter => {
                    if let crate::ui::widgets::MenuType::NewGame { .. } = menu.menu_type {
                        println!("Character name set!");
                    } else {
                        if !menu.select(state)? {
                            return Ok(false);
                        }
                    }
                }
                KeyCode::Backspace => {
                    if let crate::ui::widgets::MenuType::NewGame { .. } = menu.menu_type {
                        menu.handle_input('\x08'); // Backspace
                    }
                }
                KeyCode::Char(c) => {
                    if let crate::ui::widgets::MenuType::NewGame { .. } = menu.menu_type {
                        menu.handle_input(c); // Send typed char to menu
                    }
                }
                _ => {}
            }
        }

        Ok(true)
    }
}
