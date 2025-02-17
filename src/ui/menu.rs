use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::ListItem;

// Consts for menu options
const OPTIONS_MAIN_MENU: [&str; 3] = ["New Game", "Load Game", "Exit"];
const OPTIONS_CONFIRM: [&str; 2] = ["Enter to Confirm", "Esc to Cancel"];
const OPTIONS_CONTINUE: [&str; 1] = ["Enter to Continue"];
const OPTIONS_GAME: [&str; 4] = ["Time", "Weather", "Save", "Quit"];
const OPTIONS_GAME_QUIT: [&str; 2] = ["Yes", "No"];

// Struct for Menu
pub struct Menu {
    menu_options: Vec<String>,
    pub selected_index: usize,
}

// Functions for Menu
impl Menu {
    // Create a new Menu, defaults to Main Menu
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

    // Updates the menu options based on current state
    pub fn update(&mut self, managers: &super::display::Managers) {
        self.menu_options.clear();

        let options: &[&str] = match managers.state_manager.current_state {
            // Main Menu
            crate::core::states::StateType::MainMenu => &OPTIONS_MAIN_MENU,
            // New Game
            crate::core::states::StateType::Name | crate::core::states::StateType::NameConfirm => {
                &OPTIONS_CONFIRM
            }
            // Game, Time, and Weather
            crate::core::states::StateType::Game
            | crate::core::states::StateType::Time
            | crate::core::states::StateType::Weather => &OPTIONS_GAME,
            // Save Game and Load Game (Error)
            crate::core::states::StateType::GameSaveSuccess
            | crate::core::states::StateType::GameSaveError
            | crate::core::states::StateType::GameLoadError => &OPTIONS_CONTINUE,
            // Quit Game
            crate::core::states::StateType::GameQuit => &OPTIONS_GAME_QUIT,
        };

        self.menu_options
            .extend(options.iter().map(|&option| option.to_string()));
    }

    // Renders the Menu based on current state
    pub fn render(&self, managers: &super::display::Managers) -> Vec<ListItem> {
        match managers.state_manager.current_state {
            // New Game, Save Game, and Load Game (Error)
            crate::core::states::StateType::Name
            | crate::core::states::StateType::NameConfirm
            | crate::core::states::StateType::GameSaveSuccess
            | crate::core::states::StateType::GameSaveError
            | crate::core::states::StateType::GameLoadError => {
                let list: Vec<ListItem> = self
                    .menu_options
                    .iter()
                    .map(|option| ListItem::new(option.clone()).style(Style::default()))
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
