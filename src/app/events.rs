use crossterm::event::{self, Event, KeyCode};
use std::io;

// Struct for event handler
pub struct EventHander {}

// Functions for event handler
impl EventHander {
    // Updates how events are handled based on current state
    pub fn update(
        state_manager: &mut super::states::StateManager,
        menu: &mut crate::ui::menu::Menu,
        viewport: &mut crate::ui::viewport::Viewport,
        popup: &mut crate::ui::popup::Popup,
    ) -> io::Result<bool> {
        match state_manager.current_state {
            // New Game
            super::states::StateType::NewGame => {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) => popup.input.push(c),
                        KeyCode::Backspace => {
                            popup.input.pop();
                        }
                        KeyCode::Enter => todo!(),
                        KeyCode::Esc => {
                            popup.input.clear();
                            state_manager.current_state = state_manager.last_state.clone()
                        }
                        _ => {}
                    }
                }

                Ok(true)
            }
            // All other states
            _ => {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Up => menu.previous(),
                        KeyCode::Down => menu.next(),
                        KeyCode::Enter => {
                            if !select(state_manager, menu)? {
                                return Ok(false);
                            }
                        }
                        _ => {}
                    }
                }

                Ok(true)
            }
        }
    }
}

// Select the currently highlighted menu option
// Menu options are displayed based on current menu type
pub fn select(
    state_manager: &mut crate::app::states::StateManager,
    menu: &mut crate::ui::menu::Menu,
) -> io::Result<bool> {
    match menu.menu_type {
        // Main Menu
        crate::ui::menu::MenuType::MainMenu => match menu.highlighted() {
            "New Game" => {
                state_manager.current_state = crate::app::states::StateType::NewGame;
                state_manager.last_state = crate::app::states::StateType::MainMenu;
            }
            "Exit" => return Ok(false),
            _ => {}
        },
        // All other menus
        _ => {}
    }

    Ok(true)
}
