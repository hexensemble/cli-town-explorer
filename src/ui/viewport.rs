use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};

// Enum for viewport types
pub enum ViewportType {
    MainMenu,
    NewGame,
}

// Struct for viewport
pub struct Viewport {
    pub viewport_type: ViewportType,
}

// Functions for viewport
impl Viewport {
    // Create a new viewport, defaults to Main Menu
    pub fn new() -> Self {
        Self {
            viewport_type: ViewportType::MainMenu,
        }
    }

    // Updates the viewport type based on current state
    pub fn update(&mut self, state_manager: &crate::app::states::StateManager) {
        match state_manager.current_state {
            // Main Menu
            crate::app::states::StateType::MainMenu => {
                self.viewport_type = ViewportType::MainMenu;
            }
            // New Game
            crate::app::states::StateType::NewGame => {
                self.viewport_type = ViewportType::NewGame;
            }
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
                    "main menu".into(),
                ];

                text
            }
            // New Game
            crate::app::states::StateType::NewGame => {
                let text = vec![Line::from("Name thyself...".yellow()), Line::from(">")];

                text
            }
        }
    }
}
