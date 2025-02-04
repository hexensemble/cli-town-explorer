use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::ListItem;
use std::io;

// Consts for menu options
const OPTIONS_MAIN_MENU: [&str; 2] = ["New Game", "Exit"];
const OPTIONS_NEW_GAME: [&str; 1] = ["Cancel"];

// Enum for menu types
pub enum MenuType {
    MainMenu,
    NewGame { name_input: String },
}

// Struct for menu
pub struct Menu {
    menu_type: MenuType,
    options: Vec<String>,
    selected_index: usize,
}

// Functions for menu
impl Menu {
    // Create a new menu, defaults to Main Menu
    pub fn new() -> Self {
        Self {
            menu_type: MenuType::MainMenu,
            options: OPTIONS_MAIN_MENU.iter().map(|&s| s.into()).collect(),
            selected_index: 0,
        }
    }

    // Move cursor down the menu options
    pub fn next(&mut self) {
        if self.selected_index < self.options.len() - 1 {
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
    fn highlighted(&self) -> &str {
        &self.options[self.selected_index]
    }

    // Select the currently highlighted menu option
    // Menu options are displayed based on current menu type
    // Menu type is determined by the update function which uses current state
    pub fn select(&self, state_manager: &mut crate::app::states::StateManager) -> io::Result<bool> {
        match self.menu_type {
            // Main Menu
            MenuType::MainMenu => match self.highlighted() {
                "New Game" => {
                    state_manager.current_state = crate::app::states::StateType::NewGame;
                    state_manager.last_state = crate::app::states::StateType::MainMenu;
                }
                "Exit" => return Ok(false),
                _ => {}
            },
            // New Game
            MenuType::NewGame { name_input: _ } => match self.highlighted() {
                "Cancel" => state_manager.current_state = state_manager.last_state.clone(),
                _ => {}
            },
        }

        Ok(true)
    }

    // Updates the menu type and options based on current state
    pub fn update(&mut self, state_manager: &crate::app::states::StateManager) {
        match state_manager.current_state {
            // Main Menu
            crate::app::states::StateType::MainMenu => {
                self.menu_type = MenuType::MainMenu;
                self.options = OPTIONS_MAIN_MENU.iter().map(|&s| s.into()).collect();
            }
            // New Game
            crate::app::states::StateType::NewGame => {
                self.menu_type = MenuType::NewGame {
                    name_input: String::new(),
                };
                self.options = OPTIONS_NEW_GAME.iter().map(|&s| s.into()).collect();
            }
        }
    }

    // Renders the menu based on current State
    pub fn render(&self, state_manager: &crate::app::states::StateManager) -> Vec<ListItem> {
        match state_manager.current_state {
            // Main Menu
            crate::app::states::StateType::MainMenu => {
                let list: Vec<ListItem> = self
                    .options
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
            // New Game
            crate::app::states::StateType::NewGame => {
                let list: Vec<ListItem> = self
                    .options
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
