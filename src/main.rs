mod app;
mod ui;

// Main function
fn main() {
    // Starts Ratatui and in turn the main loop
    match ui::display::start() {
        Ok(()) => println!("Ok"),
        Err(e) => println!("Error: {}", e),
    }
}
