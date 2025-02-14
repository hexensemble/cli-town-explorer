use ratatui::style::Stylize;
use ratatui::text::Line;

// Struct for Pop Up
pub struct Popup {
    pub display: bool,
    title: String,
    pub input: String,
}

// Functions for Pop Up
impl Popup {
    // Create a new Pop Up
    pub fn new() -> Self {
        Self {
            display: false,
            title: String::new(),
            input: String::new(),
        }
    }

    // Updates if the Pop Up should be displayed based on current_state
    pub fn update(&mut self, managers: &super::display::Managers) {
        match managers.state_manager.current_state {
            // New Game, Save Game, and Quit Game
            crate::core::states::StateType::Name
            | crate::core::states::StateType::NameConfirm
            | crate::core::states::StateType::GameSaveSuccess
            | crate::core::states::StateType::GameSaveError
            | crate::core::states::StateType::GameQuit => self.display = true,

            // All other states
            _ => {
                self.display = false;
            }
        }
    }

    // Renders the Pop Up based on current state
    pub fn render(&mut self, managers: &super::display::Managers) -> (String, Vec<Line>) {
        match managers.state_manager.current_state {
            // New Game (Enter Name)
            crate::core::states::StateType::Name => {
                self.title = "New Game".into();
                let title = self.title.clone();

                let prompt = format!("> {}_", self.input);

                let text = vec![
                    Line::from("\n"),
                    Line::from("Name thyself..."),
                    Line::from(prompt.yellow()),
                ];

                (title, text)
            }
            // New Game (Confirm Name)
            crate::core::states::StateType::NameConfirm => {
                self.title = "New Game".into();
                let title = self.title.clone();

                let name = self.input.to_string();

                let text = vec![
                    Line::from("\n"),
                    Line::from("Confirm name..."),
                    Line::from(name.yellow()),
                ];

                (title, text)
            }
            // Save Game (Success)
            crate::core::states::StateType::GameSaveSuccess => {
                self.title = "Game Saved".into();
                let title = self.title.clone();

                let text = vec![Line::from("\n"), Line::from("Game saved successfully.")];

                (title, text)
            }
            // Save Game (Error)
            crate::core::states::StateType::GameSaveError => {
                self.title = "Error!".into();
                let title = self.title.clone();

                let text = vec![Line::from("\n"), Line::from("Error saving game!".red())];

                (title, text)
            }
            // Quit Game
            crate::core::states::StateType::GameQuit => {
                self.title = "Quit Game".into();
                let title = self.title.clone();

                let text = vec![
                    Line::from("\n"),
                    Line::from("Are you sure you want to quit?"),
                ];

                (title, text)
            }
            // All other states
            _ => {
                self.title = String::new();
                let title = self.title.clone();

                let text = vec![Line::from("")];

                (title, text)
            }
        }
    }
}
