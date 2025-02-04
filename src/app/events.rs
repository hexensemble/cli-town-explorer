use crossterm::event::{self, Event, KeyCode};
use std::io;

// Struct for event handler
pub struct EventHander {}

// Functions for event handler
impl EventHander {
    // Updates how events are handled based on current state
    pub fn update(
        state_manger: &mut super::states::StateManager,
        menu: &mut crate::ui::menu::Menu,
        viewport: &mut crate::ui::viewport::Viewport,
    ) -> io::Result<bool> {
        match state_manger.current_state {
            // New Game
            super::states::StateType::NewGame => {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Up => menu.previous(),
                        KeyCode::Down => menu.next(),
                        KeyCode::Enter => {
                            if !menu.select(state_manger)? {
                                return Ok(false);
                            }
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
                            if !menu.select(state_manger)? {
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
