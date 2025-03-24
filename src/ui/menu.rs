use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::ListItem;

// Consts for menu options
const OPTIONS_MAIN_MENU: [&str; 3] = ["New Game", "Load Game", "Exit"];
const OPTIONS_CONFIRM: [&str; 2] = ["Enter to Confirm", "Esc to Cancel"];
const OPTIONS_CONTINUE: [&str; 1] = ["Enter to Continue"];
const OPTIONS_GAME: [&str; 6] = [
    "Time",
    "Weather",
    "Travel to Town",
    "Travel to Building",
    "Save",
    "Quit",
];
const OPTIONS_GAME_QUIT: [&str; 2] = ["Yes", "No"];

// Struct for Menu
pub struct Menu {
    pub menu_options: Vec<String>,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub visible_count: usize,
}

// Functions for Menu
impl Menu {
    // Create a new Menu, defaults to Main Menu
    pub fn new() -> Self {
        Self {
            menu_options: OPTIONS_MAIN_MENU.iter().map(|&s| s.into()).collect(),
            selected_index: 0,
            scroll_offset: 0,
            visible_count: 0,
        }
    }

    // Move cursor down the menu options
    pub fn next(&mut self) {
        if self.selected_index < self.menu_options.len() - 1 {
            self.selected_index += 1;
        }

        if self.selected_index >= self.scroll_offset + self.visible_count {
            self.scroll_offset += 1;
        }
    }

    // Move cursor up the menu options
    pub fn previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }

        if self.selected_index < self.scroll_offset {
            self.scroll_offset -= self.scroll_offset.saturating_sub(1);
        }
    }

    // Updates the menu options based on current state
    pub fn update(&mut self, managers: &super::display::Managers) {
        self.menu_options.clear();

        match managers.state_manager.current_state {
            // Main Menu
            crate::core::states::StateType::MainMenu => {
                self.menu_options
                    .extend(OPTIONS_MAIN_MENU.iter().map(|&option| option.to_string()));
            }
            // New Game
            crate::core::states::StateType::Name | crate::core::states::StateType::NameConfirm => {
                self.menu_options
                    .extend(OPTIONS_CONFIRM.iter().map(|&option| option.to_string()));
            }
            // Game, Time, and Weather
            crate::core::states::StateType::Game
            | crate::core::states::StateType::Time
            | crate::core::states::StateType::Weather => {
                self.menu_options
                    .extend(OPTIONS_GAME.iter().map(|&option| option.to_string()));
            }
            // Travel Town
            crate::core::states::StateType::TravelTown => {
                if let Some(world) = managers.world_manager.world.as_ref() {
                    self.menu_options
                        .extend(world.towns.values().map(|town| town.name.clone()));
                    self.menu_options.push("Back".into());
                }
            }
            // Travel Building
            crate::core::states::StateType::TravelBuilding => {
                if let Some(player) = managers.world_manager.player.as_ref() {
                    if let Some(world) = managers.world_manager.world.as_ref() {
                        if let Some(town) = world.towns.get(&player.current_town_id).as_ref() {
                            self.menu_options.extend(
                                town.buildings.iter().map(|building| building.name.clone()),
                            );
                            self.menu_options.push("Back".into());
                        }
                    }
                }
            }
            // Save Game, Load Game (Error), and Initialize Game (Error)
            crate::core::states::StateType::GameSaveSuccess
            | crate::core::states::StateType::GameSaveError
            | crate::core::states::StateType::GameLoadError
            | crate::core::states::StateType::GameInitError => {
                self.menu_options
                    .extend(OPTIONS_CONTINUE.iter().map(|&option| option.to_string()));
            } // Quit Game
            crate::core::states::StateType::GameQuit => {
                self.menu_options
                    .extend(OPTIONS_GAME_QUIT.iter().map(|&option| option.to_string()));
            }
        };
    }

    // Renders the Menu based on current state
    pub fn render(&self, managers: &super::display::Managers) -> Vec<ListItem> {
        match managers.state_manager.current_state {
            // New Game, Save Game, Load Game (Error), and Initialize Game (Error)
            crate::core::states::StateType::Name
            | crate::core::states::StateType::NameConfirm
            | crate::core::states::StateType::GameSaveSuccess
            | crate::core::states::StateType::GameSaveError
            | crate::core::states::StateType::GameLoadError
            | crate::core::states::StateType::GameInitError => {
                let list: Vec<ListItem> = self
                    .menu_options
                    .iter()
                    .map(|option| ListItem::new(option.clone()).style(Style::default()))
                    .collect();

                list
            }
            // All other states
            _ => {
                let end = (self.scroll_offset + self.visible_count).min(self.menu_options.len());
                let visible_items = &self.menu_options[self.scroll_offset..end];

                let list: Vec<ListItem> = visible_items
                    .iter()
                    .enumerate()
                    .map(|(i, option)| {
                        let absolute_index = self.scroll_offset + i;
                        let style = if absolute_index == self.selected_index {
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
