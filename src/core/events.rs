use crossterm::event::{self, Event, KeyCode};
use std::io;
use std::time::Duration;

// Struct for event handler
pub struct EventHander {}

// Functions for event handler
impl EventHander {
    // Updates how events are handled based on current state
    pub fn update(
        state_manager: &mut super::states::StateManager,
        world_manager: &mut crate::world::manager::WorldManager,
        time_manager: &crate::world::time::TimeManger,
        menu: &mut crate::ui::menu::Menu,
        viewport: &mut crate::ui::viewport::Viewport,
        _stats: &mut crate::ui::stats::Stats,
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
            // New Game - Confirm name
            super::states::StateType::NameConfirm => {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Enter => {
                            world_manager.player = Some(crate::entities::player::Player::new(
                                666,
                                popup.input.clone(),
                            ));

                            popup.input.clear();

                            viewport.time_arc_rwlock = Some(time_manager.start());

                            state_manager.current_state = super::states::StateType::Game;
                        }
                        KeyCode::Esc => {
                            popup.input.clear();

                            state_manager.current_state = super::states::StateType::Name;
                        }
                        _ => {}
                    }
                }

                Ok(true)
            } // All other states
            _ => {
                if event::poll(Duration::from_millis(100))? {
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
                }

                Ok(true)
            }
        }
    }
}

// Select the currently highlighted menu option
pub fn select(
    state_manager: &mut crate::core::states::StateManager,
    menu: &mut crate::ui::menu::Menu,
) -> io::Result<bool> {
    match state_manager.current_state {
        // Main Menu
        super::states::StateType::MainMenu => match menu.selected_index {
            0 => {
                state_manager.current_state = crate::core::states::StateType::Name;
                menu.selected_index = 0;
            }
            1 => return Ok(false),
            _ => {}
        },
        // Game and Time
        super::states::StateType::Game | super::states::StateType::Time => {
            match menu.selected_index {
                0 => {
                    state_manager.current_state = super::states::StateType::Time;
                    menu.selected_index = 0;
                }
                1 => {
                    state_manager.current_state = super::states::StateType::GameQuit;
                    menu.selected_index = 0;
                }
                _ => {}
            }
        }
        // Quit Game - Confirm
        super::states::StateType::GameQuit => match menu.selected_index {
            0 => {
                state_manager.current_state = crate::core::states::StateType::MainMenu;
                menu.selected_index = 0;
            }
            1 => {
                state_manager.current_state = crate::core::states::StateType::Game;
                menu.selected_index = 0;
            }
            _ => {}
        }, // All other states
        _ => {}
    }

    Ok(true)
}
