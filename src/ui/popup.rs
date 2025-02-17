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

    // Updates the Pop Up based on current_state
    pub fn update(&mut self, managers: &super::display::Managers) {
        match managers.state_manager.current_state {
            // New Game
            crate::core::states::StateType::Name | crate::core::states::StateType::NameConfirm => {
                self.display = true;
                self.title = "New Game".into();
            }
            // Save Game (Success)
            crate::core::states::StateType::GameSaveSuccess => {
                self.display = true;
                self.title = "Game Saved".into();
            }
            // Save Game (Error)
            crate::core::states::StateType::GameSaveError => {
                self.display = true;
                self.title = "Save Error!".into();
            }
            // Load Game (Error)
            crate::core::states::StateType::GameLoadError => {
                self.display = true;
                self.title = "Load Error!".into();
            }
            // Quit Game
            crate::core::states::StateType::GameQuit => {
                self.display = true;
                self.title = "Quit Game".into();
            }
            // All other states
            _ => {
                self.display = false;
                self.title = String::new();
            }
        }
    }

    // Renders the Pop Up based on current state
    pub fn render(&self, managers: &super::display::Managers) -> (String, Vec<Line>) {
        match managers.state_manager.current_state {
            // New Game (Enter Name)
            crate::core::states::StateType::Name => {
                let title = &self.title;

                let prompt = format!("> {}_", self.input);

                let text = vec![
                    Line::from("\n"),
                    Line::from("Name thyself..."),
                    Line::from(prompt.yellow()),
                ];

                (title.to_string(), text)
            }
            // New Game (Confirm Name)
            crate::core::states::StateType::NameConfirm => {
                let title = &self.title;

                let name = self.input.to_string();

                let text = vec![
                    Line::from("\n"),
                    Line::from("Confirm name..."),
                    Line::from(name.yellow()),
                ];

                (title.to_string(), text)
            }
            // Save Game (Success)
            crate::core::states::StateType::GameSaveSuccess => {
                let title = &self.title;

                let text = vec![Line::from("\n"), Line::from("Game saved successfully.")];

                (title.to_string(), text)
            }
            // Save Game (Error)
            crate::core::states::StateType::GameSaveError => {
                let title = &self.title;

                let text = vec![Line::from("\n"), Line::from("Error saving game!".red())];

                (title.to_string(), text)
            }
            // Load Game (Error)
            crate::core::states::StateType::GameLoadError => {
                let title = &self.title;

                let text = vec![Line::from("\n"), Line::from("Error loading game!".red())];

                (title.to_string(), text)
            } // Quit Game
            crate::core::states::StateType::GameQuit => {
                let title = &self.title;

                let text = vec![
                    Line::from("\n"),
                    Line::from("Are you sure you want to quit?"),
                ];

                (title.to_string(), text)
            }
            // All other states
            _ => {
                let title = &self.title;

                let text = vec![Line::from("")];

                (title.to_string(), text)
            }
        }
    }
}
