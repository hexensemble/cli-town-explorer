use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::ListItem;

// Consts for menu options
const OPTIONS_MAIN_MENU: [&str; 2] = ["New Game", "Exit"];
const OPTIONS_NAME: [&str; 2] = ["Enter to Confirm", "Esc to Cancel"];
const OPTIONS_NAME_CONFIRM: [&str; 2] = ["Enter to Confirm", "Esc to Cancel"];
const OPTIONS_GAME: [&str; 2] = ["Time", "Quit"];
const OPTIONS_GAME_QUIT: [&str; 2] = ["Yes", "No"];

// Struct for menu
pub struct Menu {
    menu_options: Vec<String>,
    pub selected_index: usize,
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

    // Updates the menu type and options based on current state
    pub fn update(&mut self, state_manager: &crate::core::states::StateManager) {
        match state_manager.current_state {
            // Main Menu
            crate::core::states::StateType::MainMenu => {
                self.menu_options = OPTIONS_MAIN_MENU.iter().map(|&s| s.into()).collect();
            }
            // New Game - Enter name
            crate::core::states::StateType::Name => {
                self.menu_options = OPTIONS_NAME.iter().map(|&s| s.into()).collect();
            }
            // New Game - Confirm name
            crate::core::states::StateType::NameConfirm => {
                self.menu_options = OPTIONS_NAME_CONFIRM.iter().map(|&s| s.into()).collect();
            }
            // Game and Time
            crate::core::states::StateType::Game | crate::core::states::StateType::Time => {
                self.menu_options = OPTIONS_GAME.iter().map(|&s| s.into()).collect();
            }
            // Quit Game - Confirm
            crate::core::states::StateType::GameQuit => {
                self.menu_options = OPTIONS_GAME_QUIT.iter().map(|&s| s.into()).collect();
            }
        }
    }

    // Renders the menu based on current State
    pub fn render(&self, state_manager: &crate::core::states::StateManager) -> Vec<ListItem> {
        match state_manager.current_state {
            // New Game
            crate::core::states::StateType::Name | crate::core::states::StateType::NameConfirm => {
                let list: Vec<ListItem> = self
                    .menu_options
                    .iter()
                    .map(|option| {
                        ListItem::new(option.clone()).style(Style::default().fg(Color::Green))
                    })
                    .collect();

                list
            }
            // All other states
            _ => {
                let list: Vec<ListItem> = self
                    .menu_options
                    .iter()
                    .enumerate()
                    .map(|(i, option)| {
                        let style = if i == self.selected_index {
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::Green)
                        };
                        ListItem::new(option.clone()).style(style)
                    })
                    .collect();

                list
            }
        }
    }
}
