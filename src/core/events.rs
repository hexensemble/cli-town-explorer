use crossterm::event::{self, Event, KeyCode};
use std::io;
use std::time::Duration;

// Struct for Event Handler
pub struct EventHander {}

// Functions for Event Handler
impl EventHander {
    // Updates how events are handled based on current state
    pub fn update(
        managers: &mut crate::ui::display::Managers,
        ui_components: &mut crate::ui::display::UIComponents,
    ) -> io::Result<bool> {
        match managers.state_manager.current_state {
            // New Game (Enter Name)
            super::states::StateType::Name => {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) => ui_components.popup.input.push(c),
                        KeyCode::Backspace => {
                            ui_components.popup.input.pop();
                        }
                        KeyCode::Enter => {
                            managers.state_manager.current_state =
                                super::states::StateType::NameConfirm;
                        }
                        KeyCode::Esc => {
                            ui_components.popup.input.clear();

                            managers.state_manager.current_state =
                                super::states::StateType::MainMenu;
                        }
                        _ => {}
                    }
                }

                Ok(true)
            }
            // New Game (Confirm Name)
            super::states::StateType::NameConfirm => {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Enter => {
                            // Start the game
                            start_game(managers, ui_components);

                            ui_components.popup.input.clear();

                            managers.state_manager.current_state = super::states::StateType::Game;
                        }
                        KeyCode::Esc => {
                            ui_components.popup.input.clear();

                            managers.state_manager.current_state = super::states::StateType::Name;
                        }
                        _ => {}
                    }
                }

                Ok(true)
            } // All other states (will use Select function)
            _ => {
                if event::poll(Duration::from_millis(100))? {
                    if let Event::Key(key) = event::read()? {
                        match key.code {
                            KeyCode::Up => ui_components.menu.previous(),
                            KeyCode::Down => ui_components.menu.next(),
                            KeyCode::Enter => {
                                if !select(managers, ui_components)? {
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
fn select(
    managers: &mut crate::ui::display::Managers,
    ui_components: &mut crate::ui::display::UIComponents,
) -> io::Result<bool> {
    match managers.state_manager.current_state {
        // Main Menu
        super::states::StateType::MainMenu => match ui_components.menu.selected_index {
            0 => {
                managers.state_manager.current_state = crate::core::states::StateType::Name;
                ui_components.menu.selected_index = 0;
            }
            1 => return Ok(false),
            _ => {}
        },
        // Game, Time, and Weather
        super::states::StateType::Game
        | super::states::StateType::Time
        | super::states::StateType::Weather => match ui_components.menu.selected_index {
            0 => {
                managers.state_manager.current_state = super::states::StateType::Time;
            }
            1 => {
                managers.state_manager.current_state = super::states::StateType::Weather;
            }
            2 => {
                managers.state_manager.current_state = super::states::StateType::GameQuit;
                ui_components.menu.selected_index = 0;
            }
            _ => {}
        },
        // Quit Game
        super::states::StateType::GameQuit => match ui_components.menu.selected_index {
            0 => {
                managers.state_manager.current_state = crate::core::states::StateType::MainMenu;
                ui_components.menu.selected_index = 0;
            }
            1 => {
                managers.state_manager.current_state = crate::core::states::StateType::Game;
                ui_components.menu.selected_index = 0;
            }
            _ => {}
        }, // All other states
        _ => {}
    }

    Ok(true)
}

// Start the game
fn start_game(
    managers: &mut crate::ui::display::Managers,
    ui_components: &mut crate::ui::display::UIComponents,
) {
    managers.world_manager.player = Some(crate::entities::player::Player::new(
        666,
        ui_components.popup.input.clone(),
    ));

    ui_components.viewport.time_arc_rwlock = Some(managers.time_manager.start());
    ui_components.viewport.weather_arc_rwlock = Some(managers.weather_manager.start());
}
