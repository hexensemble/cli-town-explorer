use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use std::fmt::Write;

// Struct for Viewport
pub struct Viewport {
    time: String,
    weather: String,
    town_name: String,
    location: String,
    room_id: String,
    list_of_buildings: String,
    list_of_rooms: String,
    list_of_npcs: String,
    list_of_containers: String,
}

// Functions for Viewport
impl Viewport {
    // Create a new Viewport
    pub fn new() -> Self {
        Self {
            time: String::new(),
            weather: String::new(),
            town_name: String::new(),
            location: String::new(),
            room_id: String::new(),
            list_of_buildings: String::new(),
            list_of_rooms: String::new(),
            list_of_npcs: String::new(),
            list_of_containers: String::new(),
        }
    }

    // Updates any dynamic parts of Viewport
    pub fn update(&mut self, managers: &super::display::Managers) {
        match managers.state_manager.current_state {
            // Game, Travel Town, Travel Building, Building, and Room
            crate::core::states::StateType::Game
            | crate::core::states::StateType::TravelTown
            | crate::core::states::StateType::TravelBuilding
            | crate::core::states::StateType::Building
            | crate::core::states::StateType::Room => {
                // Get town name
                self.town_name = if let Some(player) = managers.world_manager.player.as_ref() {
                    if let Some(world) = managers.world_manager.world.as_ref() {
                        if let Some(town) = world.towns.get(&player.current_town_id) {
                            format!("Current Town: {}.", town.name)
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
                self.location = if let Some(player) = managers.world_manager.player.as_ref() {
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

                // Get room ID
                self.room_id = if let Some(player) = managers.world_manager.player.as_ref() {
                    if let Some(room_id) = player.current_room_id.as_ref() {
                        format!("Room: {}", room_id)
                    } else {
                        "Room: None".into()
                    }
                } else {
                    "Error getting room info!".into()
                };

                // Get list of buildings
                self.list_of_buildings.clear();
                if let Some(player) = managers.world_manager.player.as_ref() {
                    if let Some(world) = managers.world_manager.world.as_ref() {
                        if let Some(town) = world.towns.get(&player.current_town_id).as_ref() {
                            for building in &town.buildings {
                                writeln!(self.list_of_buildings, "{}", building.name).unwrap();
                            }
                        } else {
                            self.list_of_buildings = "Failed to get buildings.".into();
                        }
                    } else {
                        self.list_of_buildings = "Failed to get buildings.".into();
                    }
                } else {
                    self.list_of_buildings = "Failed to get buildings.".into();
                }

                // Get list of rooms
                self.list_of_rooms.clear();
                if let Some(player) = managers.world_manager.player.as_ref() {
                    if let Some(current_building_id) = player.current_building_id.as_ref() {
                        if let Some(world) = managers.world_manager.world.as_ref() {
                            if let Some(building) =
                                world.buildings.get(current_building_id).as_ref()
                            {
                                for room in &building.rooms {
                                    writeln!(self.list_of_rooms, "{}", room.id).unwrap();
                                }
                            } else {
                                self.list_of_rooms = "Failed to get rooms.".into();
                            }
                        } else {
                            self.list_of_rooms = "Failed to get rooms.".into();
                        }
                    } else {
                        self.list_of_rooms = "Failed to get rooms.".into();
                    }
                } else {
                    self.list_of_rooms = "Failed to get rooms.".into();
                }

                //Get list of NPCs
                self.list_of_npcs.clear();
                if let Some(player) = managers.world_manager.player.as_ref() {
                    if let Some(current_room_id) = player.current_room_id.as_ref() {
                        if let Some(world) = managers.world_manager.world.as_ref() {
                            if let Some(room) = world.rooms.get(current_room_id) {
                                for npc in &room.npcs {
                                    writeln!(self.list_of_npcs, "{}", npc.name).unwrap();
                                }
                            } else {
                                self.list_of_npcs = "Failed to get NPCs.".into();
                            }
                        } else {
                            self.list_of_npcs = "Failed to get NPCs.".into();
                        }
                    } else {
                        self.list_of_npcs = "Failed to get NPCs.".into();
                    }
                } else {
                    self.list_of_npcs = "Failed to get NPCs.".into();
                }

                //Get list of containers
                self.list_of_containers.clear();
                if let Some(player) = managers.world_manager.player.as_ref() {
                    if let Some(current_room_id) = player.current_room_id.as_ref() {
                        if let Some(world) = managers.world_manager.world.as_ref() {
                            if let Some(room) = world.rooms.get(current_room_id) {
                                for container in &room.containers {
                                    writeln!(
                                        self.list_of_containers,
                                        "{:?}",
                                        container.container_type
                                    )
                                    .unwrap();
                                }
                            } else {
                                self.list_of_containers = "Failed to get containers.".into();
                            }
                        } else {
                            self.list_of_containers = "Failed to get containers.".into();
                        }
                    } else {
                        self.list_of_containers = "Failed to get containers.".into();
                    }
                } else {
                    self.list_of_containers = "Failed to get containers.".into();
                }
            }
            // Time
            crate::core::states::StateType::Time => {
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
            }
            // Weather
            crate::core::states::StateType::Weather => {
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
                    Line::from("Welcome to CLI Town Explorer!"),
                    Line::from("v1.0"),
                    Line::from("by HexEnsemble"),
                    Line::from(""),
                    Line::from("This app allows you to explore worlds created with the CLI Town Generator."),
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
                let mut output_lines = vec![
                    Line::from(self.town_name.clone()),
                    Line::from(""),
                    Line::from(self.location.clone()),
                    Line::from(""),
                    Line::from("Buildings:"),
                    Line::from(""),
                ];
                output_lines.extend(
                    self.list_of_buildings
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
            // Building
            crate::core::states::StateType::Building => {
                let mut output_lines = vec![
                    Line::from(self.town_name.clone()),
                    Line::from(""),
                    Line::from(self.location.clone()),
                    Line::from(""),
                    Line::from("Rooms:"),
                    Line::from(""),
                ];
                output_lines.extend(
                    self.list_of_rooms
                        .lines()
                        .map(|line| Line::from(line.to_string())),
                );
                output_lines.push(Line::from(""));
                output_lines.push(Line::from("Select an option from the menu below..."));
                output_lines
            }
            // Room
            crate::core::states::StateType::Room => {
                let mut output_lines = vec![
                    Line::from(self.town_name.clone()),
                    Line::from(""),
                    Line::from(self.location.clone()),
                    Line::from(""),
                    Line::from(self.room_id.clone()),
                    Line::from(""),
                    Line::from("NPCs:"),
                    Line::from(""),
                ];
                output_lines.extend(
                    self.list_of_npcs
                        .lines()
                        .map(|line| Line::from(line.to_string())),
                );
                output_lines.push(Line::from(""));
                output_lines.push(Line::from("Containers:"));
                output_lines.push(Line::from(""));
                output_lines.extend(
                    self.list_of_containers
                        .lines()
                        .map(|line| Line::from(line.to_string())),
                );
                output_lines
            }
        }
    }
}
