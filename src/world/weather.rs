use rand::Rng;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter};

// Enum for weather types
#[derive(Debug, Clone, EnumCount, EnumIter)]
pub enum WeatherType {
    Sunny,
    Raining,
    Windy,
    Stormy,
    Snowing,
}

// Struct for Game Weather
#[derive(Debug, Clone)]
pub struct GameWeather {
    pub weather_type: WeatherType,
}

// Functions for Game Weather
impl GameWeather {
    // Create a new Game Weather, defaults to Sunny
    fn new() -> Self {
        Self {
            weather_type: WeatherType::Sunny,
        }
    }
}

// Struct for Weather Manager
pub struct WeatherManager {}

// Fucntions for Weather Manager
impl WeatherManager {
    // Create a new Weather Manager
    pub fn new() -> Self {
        Self {}
    }

    // Start weather, spawns in a new thread
    pub fn start(&self) -> Arc<RwLock<GameWeather>> {
        let game_weather = Arc::new(RwLock::new(GameWeather::new()));
        let game_weather_arc_clone = Arc::clone(&game_weather);

        thread::spawn(move || loop {
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

        game_weather
    }
}

// Function for dice rolls
fn roll_dice(dice_size: u32) -> u32 {
    let mut rng = rand::rng();
    rng.random_range(1..=dice_size)
}
