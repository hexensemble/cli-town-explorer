use ratatui::style::Stylize;
use ratatui::text::Line;

// Struct for popup
pub struct Popup {
    pub display: bool,
    title: String,
    pub input: String,
}

// Functions for popup
impl Popup {
    // Create a new popup
    pub fn new() -> Self {
        Self {
            display: false,
            title: String::new(),
            input: String::new(),
        }
    }

    // Updates the popup based on current_state
    pub fn update(&mut self, state_manager: &crate::core::states::StateManager) {
        match state_manager.current_state {
            // When pop up needs to be displayed
            crate::core::states::StateType::Name
            | crate::core::states::StateType::NameConfirm
            | crate::core::states::StateType::GameQuit => self.display = true,

            // All other states
            _ => {
                self.display = false;
            }
        }
    }

    // Renders the popup based on current state
    pub fn render(
        &mut self,
        state_manager: &crate::core::states::StateManager,
    ) -> (String, Vec<Line>) {
        match state_manager.current_state {
            // New Game - Enter name
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
            // New Game - Confirm name
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
            // Quit Game - Confirm
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
