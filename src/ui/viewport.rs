use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use std::fmt::Write;

// Struct for Viewport
pub struct Viewport {
    time: String,
    weather: String,
}

// Functions for Viewport
impl Viewport {
    // Create a new Viewport
    pub fn new() -> Self {
        Self {
            time: String::new(),
            weather: String::new(),
        }
    }

    // Updates any dynamic parts of Viewport
    pub fn update(&mut self, managers: &super::display::Managers) {
        match managers.state_manager.current_state {
            // Game, Time, and Weather
            crate::core::states::StateType::Game
            | crate::core::states::StateType::Time
            | crate::core::states::StateType::Weather => {
                // Get time
                if let Some(game_time) = &managers.time_manager.time_arc_rwlock {
                    if let Ok(game_time_unwrapped) = game_time.read() {
                        self.time = format!(
                            "Day: {}, Phase: {:?}, Tick: {}",
                            game_time_unwrapped.day,
                            game_time_unwrapped.phase,
                            game_time_unwrapped.tick
                        );
                    } else {
                        log::error!("Failed to read GameTime (lock poisoned?).");
                        self.time = "GameTime unavailable".into();
                    }
                } else {
                    log::error!("Failed to initialize GameTime.");
                    self.time = "GameTime unavailable".into();
                }

                // Get weather
                if let Some(game_weather) = &managers.weather_manager.weather_arc_rwlock {
                    if let Ok(game_weather_unwrapped) = game_weather.read() {
                        self.weather =
                            format!("The weather is {:?}", game_weather_unwrapped.weather_type);
                    } else {
                        log::error!("Failed to read GameWeather (lock poisoned?).");
                        self.weather = "GameWeather unavailable".into();
                    }
                } else {
                    log::error!("Failed to initialize GameWeather.");
                    self.weather = "GameWeather unavailable".into()
                }
            }
            // All other states
            _ => {}
        }
    }

    // Renders the Viewport based on current state
    pub fn render(&self, managers: &super::display::Managers) -> Vec<Line> {
        match managers.state_manager.current_state {
            // Main Menu
            crate::core::states::StateType::MainMenu => {
                vec![
                    Line::from("This CLI app allows you to explore worlds created with the CLI Town Generator."),
                    Line::from(""),
                    Line::from(vec![
                        Span::raw("Select "),
                        Span::styled("New Game", Style::new().green().bold()),
                        Span::raw(" when you're ready to begin."),
    ]),
                ]
            }
            // New Game (Enter Name)
            crate::core::states::StateType::Name => {
                vec![Line::from("Name thyself...")]
            }
            // New Game (Confirm Name)
            crate::core::states::StateType::NameConfirm => {
                vec![Line::from("Confirm name...")]
            }
            // Game
            crate::core::states::StateType::Game => {
                // Get town name
                let town_name = if let Some(player) = managers.world_manager.player.as_ref() {
                    if let Some(world) = managers.world_manager.world.as_ref() {
                        if let Some(town) = world.towns.get(&player.current_town_id) {
                            format!("You are currently in the town of {}.", town.name)
                        } else {
                            format!(
                                "Failed to find town name for town ID: {}",
                                player.current_town_id
                            )
                        }
                    } else {
                        format!(
                            "Failed to find town name for town ID: {}",
                            player.current_town_id
                        )
                    }
                } else {
                    "Error getting town info!".into()
                };

                // Get location
                let location = if let Some(player) = managers.world_manager.player.as_ref() {
                    if let Some(current_building_id) = player.current_building_id.as_ref() {
                        if let Some(world) = managers.world_manager.world.as_ref() {
                            if let Some(building) = world.buildings.get(current_building_id) {
                                format!("Location: {}", building.name)
                            } else {
                                format!(
                                    "Failed to find building name for building ID: {}",
                                    current_building_id
                                )
                            }
                        } else {
                            format!(
                                "Failed to find building name for building ID: {}",
                                current_building_id
                            )
                        }
                    } else {
                        "Location: Outside".into()
                    }
                } else {
                    "Error getting location info!".into()
                };

                // Get list of buildings
                let mut list_of_buildings = String::new();
                if let Some(player) = managers.world_manager.player.as_ref() {
                    if let Some(world) = managers.world_manager.world.as_ref() {
                        if let Some(town) = world.towns.get(&player.current_town_id).as_ref() {
                            for building in &town.buildings {
                                writeln!(list_of_buildings, "{}", building.name).unwrap();
                            }
                        } else {
                            list_of_buildings = "Failed to get buildings.".into();
                        }
                    } else {
                        list_of_buildings = "Failed to get buildings.".into();
                    }
                } else {
                    list_of_buildings = "Failed to get buildings.".into();
                }

                // Text to render
                let mut output_lines = vec![
                    Line::from(town_name),
                    Line::from(""),
                    Line::from(location),
                    Line::from(""),
                    Line::from("Buildings:"),
                    Line::from(""),
                ];
                output_lines.extend(
                    list_of_buildings
                        .lines()
                        .map(|line| Line::from(line.to_string())),
                );
                output_lines.push(Line::from(""));
                output_lines.push(Line::from("Select an option from the menu below..."));
                output_lines
            }
            // Save Game (Success)
            crate::core::states::StateType::GameSaveSuccess => {
                vec![Line::from("Game saved successfully.")]
            }
            // Save Game (Error)
            crate::core::states::StateType::GameSaveError => {
                vec![Line::from(vec![Span::styled(
                    "Error saving game!",
                    Style::new().red(),
                )])]
            }
            // Load Game (Error)
            crate::core::states::StateType::GameLoadError => {
                vec![Line::from(vec![Span::styled(
                    "Error loading game!",
                    Style::new().red(),
                )])]
            }
            // Initialize Game (Error)
            crate::core::states::StateType::GameInitError => {
                vec![Line::from(vec![Span::styled(
                    "Error initializing game!",
                    Style::new().red(),
                )])]
            }
            // Quit Game
            crate::core::states::StateType::GameQuit => {
                vec![Line::from("Are you sure you want to quit?")]
            }
            // Time
            crate::core::states::StateType::Time => {
                vec![Line::from(self.time.clone())]
            }
            // Weather
            crate::core::states::StateType::Weather => {
                vec![Line::from(self.weather.clone())]
            }
            // Travel Town
            crate::core::states::StateType::TravelTown => {
                vec![Line::from("Which town would you like to visit?")]
            }
            // Travel Building
            crate::core::states::StateType::TravelBuilding => {
                vec![Line::from("Which building would you like to visit?")]
            }
        }
    }
}
