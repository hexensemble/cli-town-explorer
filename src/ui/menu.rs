use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::ListItem;

// Consts for menu options
const OPTIONS_MAIN_MENU: [&str; 2] = ["New Game", "Exit"];
const OPTIONS_NAME: [&str; 1] = ["Esc to Cancel"];
const OPTIONS_NAME_CONFIRM: [&str; 2] = ["Confirm", "Cancel"];

// Struct for menu
pub struct Menu {
    menu_options: Vec<String>,
    selected_index: usize,
}

// Functions for menu
impl Menu {
    // Create a new menu, defaults to Main Menu
    pub fn new() -> Self {
        Self {
            menu_options: OPTIONS_MAIN_MENU.iter().map(|&s| s.into()).collect(),
            selected_index: 0,
        }
    }

    // Move cursor down the menu options
    pub fn next(&mut self) {
        if self.selected_index < self.menu_options.len() - 1 {
            self.selected_index += 1;
        }
    }

    // Move cursor up the menu options
    pub fn previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    // Currently highlighted menu option
    pub fn highlighted(&self) -> usize {
        self.selected_index
    }

    // Updates the menu type and options based on current state
    pub fn update(&mut self, state_manager: &crate::app::states::StateManager) {
        match state_manager.current_state {
            // Main Menu
            crate::app::states::StateType::MainMenu => {
                self.menu_options = OPTIONS_MAIN_MENU.iter().map(|&s| s.into()).collect();
            }
            // New Game - Enter name
            crate::app::states::StateType::Name => {
                self.menu_options = OPTIONS_NAME.iter().map(|&s| s.into()).collect();
            }
            // New Game - Confirm name
            crate::app::states::StateType::NameConfirm => {
                self.menu_options = OPTIONS_NAME_CONFIRM.iter().map(|&s| s.into()).collect();
            }
        }
    }

    // Renders the menu based on current State
    pub fn render(&self, state_manager: &crate::app::states::StateManager) -> Vec<ListItem> {
        match state_manager.current_state {
            // All other states
            _ => {
                let list: Vec<ListItem> = self
                    .menu_options
                    .iter()
                    .enumerate()
                    .map(|(i, option)| {
                        let style = if i == self.selected_index {
                            Style::default()
                                .fg(Color::Green)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default()
                        };
                        ListItem::new(option.clone()).style(style)
                    })
                    .collect();

                list
            }
        }
    }
}
