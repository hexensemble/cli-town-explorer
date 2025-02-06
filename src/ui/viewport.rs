use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};

// Struct for viewport
pub struct Viewport {}

// Functions for viewport
impl Viewport {
    // Create a new viewport, defaults to Main Menu
    pub fn new() -> Self {
        Self {}
    }

    // Updates the viewport type based on current state
    pub fn update(&mut self, _state_manager: &crate::core::states::StateManager) {}

    // Renders the viewport based on current state
    pub fn render(&self, state_manager: &crate::core::states::StateManager) -> Vec<Line> {
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
        }
    }
}
