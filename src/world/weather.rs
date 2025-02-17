use rand::Rng;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter};

// Struct for Weather Manager
pub struct WeatherManager {
    pub weather_arc_rwlock: Option<Arc<RwLock<GameWeather>>>,
    shutdown_flag: Arc<AtomicBool>,
}

// Fucntions for Weather Manager
impl WeatherManager {
    // Create a new Weather Manager
    pub fn new() -> Self {
        Self {
            weather_arc_rwlock: None,
            shutdown_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    // Start weather, spawns in a new thread
    pub fn start(&mut self, initial_game_weather: GameWeather) {
        let game_weather = Arc::new(RwLock::new(initial_game_weather));
        let game_weather_arc_clone = Arc::clone(&game_weather);

        self.shutdown_flag.store(false, Ordering::Relaxed);
        let shutdown_flag_arc_clone = Arc::clone(&self.shutdown_flag);

        thread::spawn(move || loop {
            // Check if thread needs shutting down
            if shutdown_flag_arc_clone.load(Ordering::Relaxed) {
                break;
            }

            // Roll to see if weather changes
            if roll_dice(100) < 5 {
                let mut weather = game_weather_arc_clone.write().unwrap();

                // Roll to decide weather type
                match roll_dice(WeatherType::COUNT as u32) {
                    0 => weather.weather_type = WeatherType::Sunny,
                    1 => weather.weather_type = WeatherType::Raining,
                    2 => weather.weather_type = WeatherType::Windy,
                    3 => weather.weather_type = WeatherType::Stormy,
                    4 => weather.weather_type = WeatherType::Snowing,
                    _ => weather.weather_type = WeatherType::Sunny,
                }
            }

            // 10 second sleep to prevent excessive CPU usage
            thread::sleep(Duration::from_secs(10));
        });

        self.weather_arc_rwlock = Some(game_weather);
    }

    // Stop the weather thread
    pub fn stop(&self) {
        self.shutdown_flag.store(true, Ordering::Relaxed);
    }
}

// Function for dice rolls
fn roll_dice(dice_size: u32) -> u32 {
    let mut rng = rand::rng();
    rng.random_range(1..=dice_size)
}

// Struct for Game Weather
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameWeather {
    pub weather_type: WeatherType,
}

// Functions for Game Weather
impl GameWeather {
    // Create a new Game Weather, defaults to Sunny
    pub fn new() -> Self {
        Self {
            weather_type: WeatherType::Sunny,
        }
    }
}

// Enum for weather types
#[derive(Debug, Clone, EnumCount, EnumIter, Serialize, Deserialize)]
pub enum WeatherType {
    Sunny,
    Raining,
    Windy,
    Stormy,
    Snowing,
}
