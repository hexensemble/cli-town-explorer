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

    // Update viewport based on current state
    pub fn update(&mut self, _state_manager: &crate::core::states::StateManager) {}

    // Render viewport based on current state
    pub fn render(&mut self, state_manager: &crate::core::states::StateManager) -> Vec<Line> {
        match state_manager.current_state {
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
            // New Game - Enter name
            crate::core::states::StateType::Name => {
                vec![Line::from("Name thyself...")]
            }
            // New Game - Confirm name
            crate::core::states::StateType::NameConfirm => {
                vec![Line::from("Confirm name...")]
            }
            // Game
            crate::core::states::StateType::Game => {
                vec![Line::from("The game begins..."), 
                    Line::from("\n"), 
                    Line::from("As you’ve already seen, we have a terminal-based UI running with the help of Ratatui."), 
                    Line::from("\n"), 
                    Line::from("Select an option from the menu below...")
                ]
            }
            // Quit Game - Confirm
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
        }
    }
}
