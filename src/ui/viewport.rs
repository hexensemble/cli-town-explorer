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
    pub fn update(&mut self, state_manager: &crate::app::states::StateManager) {
        match state_manager.current_state {
            // Main Menu
            crate::app::states::StateType::MainMenu => {}
            // New Game
            crate::app::states::StateType::NewGame => {}
        }
    }

    // Renders the viewport based on current state
    pub fn render(&self, state_manager: &crate::app::states::StateManager) -> Vec<Line> {
        match state_manager.current_state {
            // Main Menu
            crate::app::states::StateType::MainMenu => {
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
            // New Game
            crate::app::states::StateType::NewGame => {
                let text = vec![Line::from("Name thyself...".yellow())];

                text
            }
        }
    }
}
