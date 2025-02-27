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
                            match start_game(managers, ui_components) {
                                Ok(()) => {
                                    ui_components.popup.input.clear();

                                    managers.state_manager.current_state =
                                        super::states::StateType::Game;
                                }
                                Err(e) => {
                                    eprintln!("Error loading game assets: {}", e);

                                    ui_components.popup.input.clear();

                                    managers.state_manager.current_state =
                                        super::states::StateType::GameInitError;
                                }
                            }
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
            // Load Game (Error) and Initialize Game (Error)
            super::states::StateType::GameLoadError | super::states::StateType::GameInitError => {
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
                managers.state_manager.current_state = super::states::StateType::Travel;
            }
            3 => {
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
            4 => {
                managers.state_manager.current_state = super::states::StateType::GameQuit;
                ui_components.menu.selected_index = 0;
            }
            _ => {}
        },
        // Travel
        super::states::StateType::Travel => match ui_components
            .menu
            .menu_options
            .get(ui_components.menu.selected_index)
        {
            Some(selected_option) => {
                if selected_option == "Back" {
                    managers.state_manager.current_state = super::states::StateType::Game;
                    ui_components.menu.selected_index = 0;
                } else if let Some(player) = managers.world_manager.player.as_mut() {
                    // Save current town
                    let current_town = player.town_name.clone();

                    // Change town to new town
                    player.town_name = selected_option.to_string();

                    // Stop time
                    managers.time_manager.stop();

                    // Save time
                    let mut time = managers
                        .time_manager
                        .time_arc_rwlock
                        .as_ref()
                        .and_then(|game_time| game_time.read().ok().map(|t| t.clone()));

                    // Advance and restart time
                    match time.as_mut() {
                        Some(time_unwrapped) => {
                            let travel_time = managers
                                .world_manager
                                .get_travel_time(&current_town, selected_option);

                            time_unwrapped.tick += travel_time;
                            time_unwrapped.day += time_unwrapped.tick / 900;
                            time_unwrapped.tick %= 900;

                            managers.time_manager.start(time_unwrapped.clone());
                        }
                        None => {
                            managers
                                .time_manager
                                .start(crate::world::time::GameTime::new());
                        }
                    }

                    managers.state_manager.current_state = super::states::StateType::Game;
                    ui_components.menu.selected_index = 0;
                }
            }
            None => {
                eprintln!("Invalid menu selection");
            }
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
) -> Result<(), Box<dyn std::error::Error>> {
    managers.world_manager.load_world()?;

    managers.world_manager.player = Some(crate::entities::player::Player::new(
        666,
        ui_components.popup.input.clone(),
        "Higashi Kawaport".into(),
    ));

    managers
        .time_manager
        .start(crate::world::time::GameTime::new());
    managers
        .weather_manager
        .start(crate::world::weather::GameWeather::new());

    Ok(())
}

// Load game from save
fn load_game(
    managers: &mut crate::ui::display::Managers,
) -> Result<(), Box<dyn std::error::Error>> {
    managers.world_manager.load_world()?;

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
