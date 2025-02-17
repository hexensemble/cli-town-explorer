use crossterm::event::{self, Event, KeyCode};
use std::io;
use std::time::Duration;

// Struct for Event Handler
pub struct EventHandler {}

// Functions for Event Handler
impl EventHandler {
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
            }
            // Save Game
            super::states::StateType::GameSaveSuccess | super::states::StateType::GameSaveError => {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Enter {
                        managers.state_manager.current_state = super::states::StateType::Game;
                    }
                }

                Ok(true)
            }
            // Load Game (Error)
            super::states::StateType::GameLoadError => {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Enter {
                        managers.state_manager.current_state = super::states::StateType::MainMenu;
                    }
                }

                Ok(true)
            }
            // All other states (these use the Select function)
            _ => {
                if event::poll(Duration::ZERO)? {
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
            1 => match load_game(managers) {
                Ok(()) => {
                    managers.state_manager.current_state = crate::core::states::StateType::Game;
                    ui_components.menu.selected_index = 0;
                }
                Err(e) => {
                    eprintln!("Unable to load save: {}", e);
                    managers.state_manager.current_state =
                        crate::core::states::StateType::GameLoadError;
                    ui_components.menu.selected_index = 0;
                }
            },
            2 => return Ok(false),
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
                match managers.save_manager.save(
                    &managers.world_manager,
                    &managers.time_manager,
                    &managers.weather_manager,
                ) {
                    Ok(()) => {
                        managers.state_manager.current_state =
                            super::states::StateType::GameSaveSuccess;
                    }
                    Err(e) => {
                        eprintln!("Error saving game: {}", e);

                        managers.state_manager.current_state =
                            super::states::StateType::GameSaveError;
                    }
                };
            }
            3 => {
                managers.state_manager.current_state = super::states::StateType::GameQuit;
                ui_components.menu.selected_index = 0;
            }
            _ => {}
        },
        // Quit Game
        super::states::StateType::GameQuit => match ui_components.menu.selected_index {
            0 => {
                managers.time_manager.stop();
                managers.weather_manager.stop();

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

    managers
        .time_manager
        .start(crate::world::time::GameTime::new());
    managers
        .weather_manager
        .start(crate::world::weather::GameWeather::new());
}

// Load game from save
fn load_game(
    managers: &mut crate::ui::display::Managers,
) -> Result<(), Box<dyn std::error::Error>> {
    let save_data = managers.save_manager.load()?;

    if let Some(player) = save_data.player {
        managers.world_manager.player = Some(player);
    } else {
        eprintln!("No player!");
    }

    if let Some(initial_game_time) = save_data.time {
        managers.time_manager.start(initial_game_time);
    } else {
        eprintln!("No GameTime!");
    }

    if let Some(initial_game_weather) = save_data.weather {
        managers.weather_manager.start(initial_game_weather);
    } else {
        eprint!("No GameWeather!");
    }

    Ok(())
}
