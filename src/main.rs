mod core;
mod entities;
mod ui;
mod world;

// Main function
fn main() {
    // Starts Ratatui and in turn the main loop
    match ui::display::start() {
        Ok(()) => println!("Ok"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
