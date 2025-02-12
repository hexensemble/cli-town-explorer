use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

// Enum for day/night phases
#[derive(Debug, Clone)]
pub enum Phase {
    Dawn,
    Day,
    Dusk,
    Night,
}

// Struct for Game Time
#[derive(Debug, Clone)]
pub struct GameTime {
    pub tick: u32,
    pub day: u32,
    pub phase: Phase,
}

// Functions for Game Time
impl GameTime {
    // Create a new Game Time, starts at dawn on the first day
    fn new() -> Self {
        Self {
            tick: 0,
            day: 1,
            phase: Phase::Dawn,
        }
    }
}

// Struct for Time Manager
pub struct TimeManger {}

// Functions for Time Manager
impl TimeManger {
    // Create a new Time Manager
    pub fn new() -> Self {
        Self {}
    }

    // Start time, spawns in new thread
    pub fn start(&self) -> Arc<RwLock<GameTime>> {
        let game_time = Arc::new(RwLock::new(GameTime::new()));
        let game_time_arc_clone = Arc::clone(&game_time);

        thread::spawn(move || {
            // Total real-world time for one in-game day (15 minutes)
            let real_time_per_day = Duration::from_secs(15 * 60);

            // Number of ticks in one day (e.g., 900 ticks for 15 minutes)
            let ticks_per_day = 900;

            // Duration of one tick in real-world time
            let tick_duration = real_time_per_day / ticks_per_day;

            // Day/Night cycle
            let dawn_ticks = (ticks_per_day as f64 * 0.25) as u32;
            let day_ticks = (ticks_per_day as f64 * 0.5) as u32;
            let dusk_ticks = (ticks_per_day as f64 * 0.75) as u32;
            let _night_ticks = (ticks_per_day as f64 * 1.0) as u32;

            // Time tracking
            let mut current_tick = 0;
            let mut current_day = 1;
            let mut current_phase = Phase::Dawn;
            let mut accumulated_time = Duration::ZERO;
            let mut last_time = Instant::now();

            loop {
                // Calculate delta time
                let now = Instant::now();
                let delta = now.duration_since(last_time);
                last_time = now;

                // Accumulate elapsed time
                accumulated_time += delta;

                // We only want to update GameTime when the time has changed
                let mut time_updated = false;

                // Process a tick if enough real world time has passed
                while accumulated_time >= tick_duration {
                    accumulated_time -= tick_duration;
                    current_tick += 1;
                    time_updated = true;

                    // Increment current day once ticks per day limit reached, reset ticks to 0
                    if current_tick == ticks_per_day {
                        current_day += 1;
                        current_tick = 0;
                    }

                    // Determine current day/night phase
                    if current_tick <= dawn_ticks {
                        current_phase = Phase::Dawn
                    } else if current_tick > dawn_ticks && current_tick <= day_ticks {
                        current_phase = Phase::Day
                    } else if current_tick > day_ticks && current_tick <= dusk_ticks {
                        current_phase = Phase::Dusk
                    } else {
                        current_phase = Phase::Night
                    }
                }

                // Only update GameTime if something changed
                if time_updated {
                    let mut time = game_time_arc_clone.write().unwrap();
                    time.tick = current_tick;
                    time.day = current_day;
                    time.phase = current_phase.clone();
                }

                // Tiny sleep to prevent excessive CPU usage
                thread::sleep(Duration::from_millis(1));
            }
        });

        game_time
    }
}
