use env_logger::Builder;
use log::LevelFilter;
use std::fs::File;
use std::io::Write;
use std::process;

mod core;
mod entities;
mod ui;
mod world;

// Main function
fn main() {
    // Initialise logging
    match init_logger() {
        Ok(()) => {
            // Starts Ratatui and in turn the main loop
            match ui::display::start() {
                Ok(()) => {
                    process::exit(0);
                }
                Err(e) => {
                    log::error!("{}", e);
                    eprintln!("{}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to initialize logging: {}", e);
            process::exit(1);
        }
    }
}

fn init_logger() -> Result<(), std::io::Error> {
    let log_file = File::create("output.log")?;

    Builder::new()
        .format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args()))
        .filter(None, LevelFilter::Info)
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .init();

    Ok(())
}
