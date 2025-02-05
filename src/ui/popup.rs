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
    pub fn update(&mut self, state_manager: &crate::app::states::StateManager) {
        match state_manager.current_state {
            // New Game - Enter name
            crate::app::states::StateType::Name => self.display = true,
            // New Game - Confirm name
            crate::app::states::StateType::NameConfirm => self.display = true,
            // All other states
            _ => {
                self.display = false;
            }
        }
    }

    // Renders the popup based on current state
    pub fn render(
        &mut self,
        state_manager: &crate::app::states::StateManager,
    ) -> (String, Vec<Line>) {
        match state_manager.current_state {
            // New Game - Enter name
            crate::app::states::StateType::Name => {
                self.title = "New Game".into();
                let title = self.title.clone();

                let prompt = format!("> {}_", self.input);

                let text = vec![Line::from("Name thyself...".yellow()), Line::from(prompt)];

                (title, text)
            }
            // New Game - Confirm name
            crate::app::states::StateType::NameConfirm => {
                self.title = "New Game".into();
                let title = self.title.clone();

                let name = format!("{}", self.input);

                let text = vec![Line::from("Your name is...".yellow()), Line::from(name)];

                (title, text)
            }
            // All other states
            _ => {
                self.title = String::new();
                let title = self.title.clone();

                let text = vec![Line::from("".yellow())];

                (title, text)
            }
        }
    }
}
