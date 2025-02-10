use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use std::sync::{Arc, RwLock};

// Struct for viewport
pub struct Viewport {
    pub time_arc_rwlock: Option<Arc<RwLock<crate::world::time::GameTime>>>,
    time: String,
}

// Functions for viewport
impl Viewport {
    // Create a new viewport, defaults to Main Menu
    pub fn new() -> Self {
        Self {
            time_arc_rwlock: None,
            time: String::new(),
        }
    }

    // Updates the viewport type based on current state
    pub fn update(&mut self, _state_manager: &crate::core::states::StateManager) {}

    // Renders the viewport based on current state
    pub fn render(&mut self, state_manager: &crate::core::states::StateManager) -> Vec<Line> {
        match state_manager.current_state {
            // Main Menu
            crate::core::states::StateType::MainMenu => {
                let text = vec![
                    Line::from(vec![
                        Span::raw("This "),
                        Span::styled("is", Style::new().green().italic()),
                        "...".into(),
                    ]),
                    Line::from("the".red()),
                    "viewport when main menu is active".into(),
                ];

                text
            }
            // New Game - Enter name
            crate::core::states::StateType::Name => {
                let text = vec![Line::from("Name thyself...".yellow())];

                text
            }
            // New Game - Confirm name
            crate::core::states::StateType::NameConfirm => {
                let text = vec![Line::from("Confirm name...".yellow())];

                text
            }
            // Game
            crate::core::states::StateType::Game => {
                let text = vec![Line::from("The game begins".yellow())];

                text
            }
            // Quit Game - Confirm
            crate::core::states::StateType::GameQuit => {
                let text = vec![Line::from("Are you sure you want to quit?".yellow())];

                text
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

                let text = vec![Line::from(self.time.clone().yellow())];

                text
            }
        }
    }
}
