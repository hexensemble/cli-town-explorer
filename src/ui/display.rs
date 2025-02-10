use color_eyre::Result;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::prelude::Rect;
use ratatui::widgets::{Block, BorderType, Borders, Clear, List, Paragraph};
use ratatui::{DefaultTerminal, Frame};

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
    let mut state_manager = crate::core::states::StateManager::new();
    let mut world_manager = crate::world::manager::WorldManager::new();
    let time_manager = crate::world::time::TimeManger::new();
    let mut menu = super::menu::Menu::new();
    let mut viewport = super::viewport::Viewport::new();
    let mut popup = super::popup::Popup::new();

    loop {
        // Update
        menu.update(&state_manager);
        viewport.update(&state_manager);
        popup.update(&state_manager);

        // Render
        terminal.draw(|frame| {
            render(frame, &state_manager, &menu, &mut viewport, &mut popup);
        })?;

        // Handle events
        if !crate::core::events::EventHander::update(
            &mut state_manager,
            &mut world_manager,
            &time_manager,
            &mut menu,
            &mut viewport,
            &mut popup,
        )? {
            break Ok(());
        }
    }
}

// Ratatui rendering
fn render(
    frame: &mut Frame,
    state_manager: &crate::core::states::StateManager,
    menu: &super::menu::Menu,
    viewport: &mut super::viewport::Viewport,
    popup: &mut super::popup::Popup,
) {
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

    // Viewport
    let viewport_text = viewport.render(state_manager);

    let viewport_block = Paragraph::new(viewport_text)
        .block(Block::default().title("Viewport").borders(Borders::ALL));
    frame.render_widget(viewport_block, horizontal[0]);

    // Menu
    let menu_options = menu.render(state_manager);

    let menu_block =
        List::new(menu_options).block(Block::default().title("Menu").borders(Borders::ALL));
    frame.render_widget(menu_block, horizontal[1]);

    // Stats
    let stats_block = Block::default().title("Stats").borders(Borders::ALL);
    frame.render_widget(stats_block, vertical[1]);

    // Popup (if required)
    if popup.display {
        let (popup_title, popup_text) = popup.render(state_manager);

        let popup_area = centered_rect(40, 10, area);

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
