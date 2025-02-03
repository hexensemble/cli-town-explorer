use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::ListItem;
use std::io;

const MENUOPTIONS_MAIN_MENU: [&str; 2] = ["New Game", "Exit"];
const MENUOPTIONS_NEW_GAME: [&str; 1] = ["Cancel"];

enum MenuType {
    MainMenu,
    NewGame,
}

pub struct MenuWidget {
    menu_type: MenuType,
    options: Vec<String>,
    selected_index: usize,
}

impl MenuWidget {
    pub fn new() -> Self {
        Self {
            menu_type: MenuType::MainMenu,
            options: MENUOPTIONS_MAIN_MENU.iter().map(|&s| s.into()).collect(),
            selected_index: 0,
        }
    }

    pub fn next(&mut self) {
        if self.selected_index < self.options.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn select(&self, state: &mut crate::app::state::State) -> io::Result<bool> {
        match self.menu_type {
            MenuType::MainMenu => match self.selected_option() {
                "New Game" => {
                    state.state_type = crate::app::state::StateType::NewGame;
                    state.last_state = crate::app::state::StateType::MainMenu;
                }
                "Exit" => return Ok(false),
                _ => {}
            },
            MenuType::NewGame => match self.selected_option() {
                "Cancel" => state.state_type = state.last_state.clone(),
                _ => {}
            },
        }

        Ok(true)
    }

    fn selected_option(&self) -> &str {
        &self.options[self.selected_index]
    }

    pub fn update(&mut self, state: &crate::app::state::State) {
        match state.state_type {
            crate::app::state::StateType::MainMenu => {
                self.menu_type = MenuType::MainMenu;
                self.options = MENUOPTIONS_MAIN_MENU.iter().map(|&s| s.into()).collect();
            }
            crate::app::state::StateType::NewGame => {
                self.menu_type = MenuType::NewGame;
                self.options = MENUOPTIONS_NEW_GAME.iter().map(|&s| s.into()).collect();
            }
        }
    }

    pub fn render(&self) -> Vec<ListItem> {
        let menu_options: Vec<ListItem> = self
            .options
            .iter()
            .enumerate()
            .map(|(i, option)| {
                let style = if i == self.selected_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(option.clone()).style(style)
            })
            .collect();

        menu_options
    }
}

enum MainType {
    MainMenu,
    NewGame,
}

pub struct MainWidget {
    main_type: MainType,
}

impl MainWidget {
    pub fn new() -> Self {
        Self {
            main_type: MainType::MainMenu,
        }
    }

    pub fn update(&mut self, state: &crate::app::state::State) {
        match state.state_type {
            crate::app::state::StateType::MainMenu => {
                self.main_type = MainType::MainMenu;
            }
            crate::app::state::StateType::NewGame => {
                self.main_type = MainType::NewGame;
            }
        }
    }

    pub fn render(&self) -> Vec<Line> {
        match self.main_type {
            MainType::MainMenu => {
                let main_text = vec![
                    Line::from(vec![
                        Span::raw("This "),
                        Span::styled("is", Style::new().green().italic()),
                        "...".into(),
                    ]),
                    Line::from("the".red()),
                    "main menu".into(),
                ];

                main_text
            }
            MainType::NewGame => {
                let main_text = vec![
                    Line::from(vec![
                        Span::raw("This "),
                        Span::styled("is", Style::new().green().italic()),
                        "...".into(),
                    ]),
                    Line::from("a".red()),
                    "new game".into(),
                ];

                main_text
            }
        }
    }
}
