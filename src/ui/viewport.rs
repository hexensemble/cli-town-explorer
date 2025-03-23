use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};

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
                    Line::from("\n"),
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
                let town_name = if let Some(player) = managers.world_manager.player.as_ref() {
                    format!("You are currently in the town of {}", player.town_name)
                } else {
                    "Error getting town info!".into()
                };

                vec![
                    Line::from(town_name),
                    Line::from("\n"),
                    Line::from("Select an option from the menu below..."),
                ]
            }
            // Save Game (Success)
            crate::core::states::StateType::GameSaveSuccess => {
                vec![
                    Line::from("Game saved successfully."),
                    Line::from("\n"),
                    Line::from(
                        "All game data has been serialized and saved to JSON file: saves/save.json",
                    ),
                ]
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
            // Travel
            crate::core::states::StateType::Travel => {
                vec![Line::from("Where would you like to go?")]
            }
        }
    }
}
