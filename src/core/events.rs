use core::panic;
use crossterm::event::{self, Event, KeyCode};
use std::collections::HashMap;
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
                                    log::error!("Failed to load game assets: {}", e);

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
                    log::error!("Failed to load save: {}", e);

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
                managers.state_manager.current_state = super::states::StateType::TravelTown;
            }
            3 => {
                managers.state_manager.current_state = super::states::StateType::TravelBuilding;
            }
            4 => {
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
                        log::error!("Failed to save game: {}", e);

                        managers.state_manager.current_state =
                            super::states::StateType::GameSaveError;
                    }
                };
            }
            5 => {
                managers.state_manager.current_state = super::states::StateType::GameQuit;
                ui_components.menu.selected_index = 0;
            }
            _ => {}
        },
        // Travel Town
        super::states::StateType::TravelTown => match ui_components
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
                    let current_town = if let Some(world) = managers.world_manager.world.as_ref() {
                        if let Some(town) = world.towns.get(&player.current_town_id) {
                            town.name.clone()
                        } else {
                            log::error!(
                                "Failed to find town name for town ID: {}",
                                player.current_town_id
                            );
                            panic!(
                                "Failed to find town name for town ID: {}",
                                player.current_town_id
                            )
                        }
                    } else {
                        log::error!(
                            "Failed to find town name for town ID: {}",
                            player.current_town_id
                        );
                        panic!(
                            "Failed to find town name for town ID: {}",
                            player.current_town_id
                        )
                    };

                    // Change town to new town
                    let towns = if let Some(world) = managers.world_manager.world.as_ref() {
                        &world.towns
                    } else {
                        log::error!("Failed to find world data while attempting to travel.");
                        panic!("Failed to find world data while attempting to travel.")
                    };

                    match find_id_by_name(towns, selected_option) {
                        Some(id) => player.current_town_id = id,
                        None => {
                            log::error!(
                                "Failed to find town ID for town name: {}",
                                selected_option
                            );
                            panic!("Failed to find town ID for town name: {}", selected_option);
                        }
                    };

                    // Move player to outside
                    player.current_building_id = None;

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
                log::error!(
                    "Failed to find town at selected index: {}",
                    ui_components.menu.selected_index
                );
            }
        },
        // Travel Building
        super::states::StateType::TravelBuilding => match ui_components
            .menu
            .menu_options
            .get(ui_components.menu.selected_index)
        {
            Some(selected_option) => {
                if selected_option == "Back" {
                    managers.state_manager.current_state = super::states::StateType::Game;
                    ui_components.menu.selected_index = 0;
                } else if let Some(player) = managers.world_manager.player.as_mut() {
                    let buildings = if let Some(world) = managers.world_manager.world.as_ref() {
                        &world.buildings
                    } else {
                        log::error!("Failed to find world data while attempting to travel.");
                        panic!("Failed to find world data while attempting to travel.")
                    };

                    player.current_building_id = find_id_by_name(buildings, selected_option);

                    managers.state_manager.current_state = super::states::StateType::Game;
                    ui_components.menu.selected_index = 0;
                }
            }
            None => {
                log::error!(
                    "Failed to find building at selectec index {}",
                    ui_components.menu.selected_index
                );
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
    managers.world_manager.clear();

    managers.world_manager.load_world()?;

    managers.world_manager.player = Some(crate::entities::player::Player::new(
        666,
        ui_components.popup.input.clone(),
        59015,
        None,
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
    managers.world_manager.clear();

    managers.world_manager.load_world()?;

    let save_data = managers.save_manager.load()?;

    if let Some(player) = save_data.player {
        managers.world_manager.player = Some(player);
    } else {
        log::error!("Failed to load Player: No Player found.");
    }

    if let Some(initial_game_time) = save_data.time {
        managers.time_manager.start(initial_game_time);
    } else {
        log::error!("Failed to load GameTime: No GameTime found.");
    }

    if let Some(initial_game_weather) = save_data.weather {
        managers.weather_manager.start(initial_game_weather);
    } else {
        log::error!("Failed to load GameWeather: No GameWeather found.");
    }

    Ok(())
}

// Trait and function for getting ID from Town or Building name
trait HasName {
    fn name(&self) -> &str;
}

impl HasName for crate::world::manager::Town {
    fn name(&self) -> &str {
        &self.name
    }
}

impl HasName for crate::world::manager::Building {
    fn name(&self) -> &str {
        &self.name
    }
}

fn find_id_by_name<'a, T: HasName>(items: &'a HashMap<u32, T>, name: &str) -> Option<u32> {
    items
        .iter()
        .find_map(|(id, item)| if item.name() == name { Some(*id) } else { None })
}
