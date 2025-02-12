use color_eyre::Result;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::prelude::Rect;
use ratatui::widgets::{Block, BorderType, Borders, Clear, List, Paragraph};
use ratatui::{DefaultTerminal, Frame};

// Struct for Managers
pub struct Managers {
    pub state_manager: crate::core::states::StateManager,
    pub world_manager: crate::world::manager::WorldManager,
    pub time_manager: crate::world::time::TimeManger,
    pub weather_manager: crate::world::weather::WeatherManager,
}

// Struct for UI Components
pub struct UIComponents {
    pub menu: super::menu::Menu,
    pub viewport: super::viewport::Viewport,
    stats: super::stats::Stats,
    pub popup: super::popup::Popup,
}

// Starts Ratatui and launches the main loop with run()
// Restores original terminal when main loop in run() finishes
pub fn start() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let result = run(terminal);

    ratatui::restore();
    result
}

// Main loop
fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut managers = Managers {
        state_manager: crate::core::states::StateManager::new(),
        world_manager: crate::world::manager::WorldManager::new(),
        time_manager: crate::world::time::TimeManger::new(),
        weather_manager: crate::world::weather::WeatherManager::new(),
    };

    let mut ui_components = UIComponents {
        menu: super::menu::Menu::new(),
        viewport: super::viewport::Viewport::new(),
        stats: super::stats::Stats::new(),
        popup: super::popup::Popup::new(),
    };

    loop {
        // Update
        ui_components.menu.update(&managers);
        ui_components.popup.update(&managers);

        // Render
        terminal.draw(|frame| {
            render(frame, &managers, &mut ui_components);
        })?;

        // Handle events
        if !crate::core::events::EventHander::update(&mut managers, &mut ui_components)? {
            break Ok(());
        }
    }
}

// Ratatui rendering
fn render(frame: &mut Frame, managers: &Managers, ui_components: &mut UIComponents) {
    // Layout
    let area = frame.area();

    let vertical = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70), // Viewport and menu
            Constraint::Percentage(30), // Stats
        ])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60), // Viewport
            Constraint::Percentage(40), // Menu
        ])
        .split(vertical[0]);

    // Menu
    let menu_options = ui_components.menu.render(managers);

    let menu_block =
        List::new(menu_options).block(Block::default().title("Menu").borders(Borders::ALL));
    frame.render_widget(menu_block, horizontal[1]);

    // Viewport
    let viewport_text = ui_components.viewport.render(managers);

    let viewport_block = Paragraph::new(viewport_text)
        .block(Block::default().title("Viewport").borders(Borders::ALL));
    frame.render_widget(viewport_block, horizontal[0]);

    // Stats
    let stats_text = ui_components.stats.render(managers);

    let stats_block =
        Paragraph::new(stats_text).block(Block::default().title("Stats").borders(Borders::ALL));
    frame.render_widget(stats_block, vertical[1]);

    // Popup (if required)
    if ui_components.popup.display {
        let (popup_title, popup_text) = ui_components.popup.render(managers);

        let popup_area = centered_rect(40, 15, area);

        let popup_block = Paragraph::new(popup_text)
            .block(
                Block::default()
                    .title(popup_title)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .alignment(Alignment::Center);
        frame.render_widget(Clear, popup_area); // Clear area under popup
        frame.render_widget(popup_block, popup_area);
    }
}

// Calculate a centered rectangle for popups
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_width = area.width * percent_x / 100;
    let popup_height = area.height * percent_y / 100;
    let x = (area.width - popup_width) / 2;
    let y = (area.height - popup_height) / 2;
    Rect::new(x, y, popup_width, popup_height)
}
