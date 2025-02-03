use color_eyre::Result;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, List, Paragraph};
use ratatui::{DefaultTerminal, Frame};

pub fn start() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut main = super::widgets::MainWidget::new();
    let mut menu = super::widgets::MenuWidget::new();
    let mut state = crate::app::state::State::new();

    loop {
        main.update(&state);
        menu.update(&state);

        terminal.draw(|frame| {
            render(frame, &main, &menu);
        })?; // terminal.draw will give you a frame, then you can do a thing with that frame, in
             // this case we call the render() function

        if !crate::app::events::EventHander::update(&mut main, &mut menu, &mut state)? {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, main: &super::widgets::MainWidget, menu: &super::widgets::MenuWidget) {
    let main_text = main.render();
    let menu_options = menu.render();

    let size = frame.area();

    let vertical = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70), // Main screen and menu
            Constraint::Percentage(30), // Stats
        ])
        .split(size);

    let horizontal = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60), //Main screen
            Constraint::Percentage(40), // Menu
        ])
        .split(vertical[0]);

    let main_block =
        Paragraph::new(main_text).block(Block::default().title("Main").borders(Borders::ALL));
    frame.render_widget(main_block, horizontal[0]);

    let menu_block =
        List::new(menu_options).block(Block::default().title("Menu").borders(Borders::ALL));
    frame.render_widget(menu_block, horizontal[1]);

    let stats_block = Block::default().title("Stats").borders(Borders::ALL);
    frame.render_widget(stats_block, vertical[1]);
}
