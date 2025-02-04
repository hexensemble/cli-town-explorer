use color_eyre::Result;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, List, Paragraph};
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
    let mut menu = super::menu::Menu::new();
    let mut viewport = super::viewport::Viewport::new();
    let mut state_manager = crate::app::states::StateManager::new();

    loop {
        // Update
        menu.update(&state_manager);
        viewport.update(&state_manager);

        // Render
        terminal.draw(|frame| {
            render(frame, &state_manager, &menu, &viewport);
        })?;

        // Handle events
        if !crate::app::events::EventHander::update(&mut state_manager, &mut menu, &mut viewport)? {
            break Ok(());
        }
    }
}

// Ratatui rendering
fn render(
    frame: &mut Frame,
    state_manager: &crate::app::states::StateManager,
    menu: &super::menu::Menu,
    viewport: &super::viewport::Viewport,
) {
    let menu_options = menu.render(state_manager);
    let viewport_text = viewport.render(state_manager);

    let size = frame.area();

    let vertical = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70), // Viewport and menu
            Constraint::Percentage(30), // Stats
        ])
        .split(size);

    let horizontal = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60), // Viewport
            Constraint::Percentage(40), // Menu
        ])
        .split(vertical[0]);

    let viewport_block = Paragraph::new(viewport_text)
        .block(Block::default().title("Viewport").borders(Borders::ALL));
    frame.render_widget(viewport_block, horizontal[0]);

    let menu_block =
        List::new(menu_options).block(Block::default().title("Menu").borders(Borders::ALL));
    frame.render_widget(menu_block, horizontal[1]);

    let stats_block = Block::default().title("Stats").borders(Borders::ALL);
    frame.render_widget(stats_block, vertical[1]);
}
