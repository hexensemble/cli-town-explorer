use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use std::sync::{Arc, RwLock};

// Struct for Viewport
pub struct Viewport {
    pub time_arc_rwlock: Option<Arc<RwLock<crate::world::time::GameTime>>>,
    pub weather_arc_rwlock: Option<Arc<RwLock<crate::world::weather::GameWeather>>>,
    time: String,
    weather: String,
}

// Functions for Viewport
impl Viewport {
    // Create a new Viewport
    pub fn new() -> Self {
        Self {
            time_arc_rwlock: None,
            weather_arc_rwlock: None,
            time: String::new(),
            weather: String::new(),
        }
    }

    // Renders the Viewport based on current state
    pub fn render(&mut self, managers: &super::display::Managers) -> Vec<Line> {
        match managers.state_manager.current_state {
            // Main Menu
            crate::core::states::StateType::MainMenu => {
                vec![
                    Line::from("This is a small CLI app designed to showcase my Rust skills in a fun and interactive way."),
                    Line::from("It’s a simple RPG where you can explore towns and interact with the world."),
                    Line::from("Features will be explained as you move about."),
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
                vec![Line::from("The game begins..."), 
                    Line::from("\n"), 
                    Line::from("As you’ve already seen, we have a terminal-based UI running with the help of Ratatui."),
                    Line::from("There are three \"sections\" which all update based on the game's current state."),
                    Line::from("\n"), 
                    Line::from("Select an option from the menu below...")
                ]
            }
            // Quit Game
            crate::core::states::StateType::GameQuit => {
                vec![Line::from("Are you sure you want to quit?")]
            }
            // Time
            crate::core::states::StateType::Time => {
                match &self.time_arc_rwlock {
                    Some(game_time) => {
                        let game_time_unwrapped = game_time.read().unwrap();
                        self.time = format!(
                            "Day: {}, Phase: {:?}, Tick: {}",
                            game_time_unwrapped.day,
                            game_time_unwrapped.phase,
                            game_time_unwrapped.tick
                        );
                    }
                    None => {
                        eprintln!("GameTime not initialized");
                        self.time = "GameTime not initialized".into();
                    }
                }

                vec![
                    Line::from(self.time.clone()),
                    Line::from("\n"),
                    Line::from("Time runs in its own thread and updates continuously."),
                ]
            }
            // Weather
            crate::core::states::StateType::Weather => {
                match &self.weather_arc_rwlock {
                    Some(weather) => {
                        let weather_unwrapped = weather.read().unwrap();
                        self.weather =
                            format!("The weather is {:?}", weather_unwrapped.weather_type)
                    }
                    None => {
                        eprintln!("Weather not initialized");
                        self.weather = "Weather not initialized".into()
                    }
                }

                vec![
                    Line::from(self.weather.clone()),
                    Line::from("\n"),
                    Line::from("Weather runs in its own thread and updates continuously."),
                ]
            }
        }
    }
}
