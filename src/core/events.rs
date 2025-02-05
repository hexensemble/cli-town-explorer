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
            // New Game - Enter name
            super::states::StateType::Name => {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) => popup.input.push(c),
                        KeyCode::Backspace => {
                            popup.input.pop();
                        }
                        KeyCode::Enter => {
                            state_manager.current_state = super::states::StateType::NameConfirm;
                            state_manager.last_state = super::states::StateType::Name;
                        }
                        KeyCode::Esc => {
                            popup.input.clear();
                            state_manager.current_state = super::states::StateType::MainMenu;
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
pub fn select(
    state_manager: &mut crate::core::states::StateManager,
    menu: &crate::ui::menu::Menu,
) -> io::Result<bool> {
    match state_manager.current_state {
        // Main Menu
        super::states::StateType::MainMenu => match menu.highlighted() {
            0 => {
                state_manager.current_state = crate::core::states::StateType::Name;
                state_manager.last_state = crate::core::states::StateType::MainMenu;
            }
            1 => return Ok(false),
            _ => {}
        },
        // New Game - Confirm name
        super::states::StateType::NameConfirm => match menu.highlighted() {
            0 => todo!(),
            1 => state_manager.current_state = crate::core::states::StateType::Name,
            _ => {}
        },
        // All other states
        _ => {}
    }

    Ok(true)
}
