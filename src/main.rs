mod app;
mod ui;

fn main() {
    match ui::display::start() {
        Ok(()) => println!("Ok"),
        Err(e) => println!("Error: {}", e),
    }
}
