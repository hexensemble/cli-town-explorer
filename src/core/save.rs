use serde::Serialize;
use serde_json;
use std::fs;

// Struct for Save Game Manager
pub struct SaveGameManager {
    save_data: SaveData,
}

// Functions for Save Game Manager
impl SaveGameManager {
    // Create a new Save Game Manager
    pub fn new() -> Self {
        Self {
            save_data: SaveData::new(),
        }
    }

    // Save the game to JSON
    pub fn save(
        &mut self,
        world_manager: &crate::world::manager::WorldManager,
        time_manager: &crate::world::time::TimeManger,
        weather_manager: &crate::world::weather::WeatherManager,
    ) -> Result<(), std::io::Error> {
        match &world_manager.player {
            Some(player) => {
                self.save_data.player = Some(player.clone());
            }
            None => {
                eprint!("Player not initialized")
            }
        }

        match &time_manager.time_arc_rwlock {
            Some(game_time) => {
                let game_time_unwrapped = game_time.read().unwrap();

                self.save_data.time = Some(crate::world::time::GameTime {
                    tick: game_time_unwrapped.tick,
                    day: game_time_unwrapped.day,
                    phase: game_time_unwrapped.phase.clone(),
                });
            }
            None => {
                eprintln!("GameTime not initialized");
            }
        }

        match &weather_manager.weather_arc_rwlock {
            Some(game_weather) => {
                let game_weather_unwrapped = game_weather.read().unwrap();

                self.save_data.weather = Some(game_weather_unwrapped.clone());
            }
            None => {
                eprint!("GameWeather not initialized");
            }
        }

        // Serialize JSON
        let json = serde_json::to_string_pretty(&self.save_data)?;

        // Save JSON to file
        let output_dir = "saves";
        let filename = "save.json";
        let filepath = format!("{}/{}", output_dir, filename);
        fs::create_dir_all(output_dir)?;
        fs::write(filepath, json)?;

        Ok(())
    }
}

// Struct for Save Data
#[derive(Serialize)]
struct SaveData {
    player: Option<crate::entities::player::Player>,
    time: Option<crate::world::time::GameTime>,
    weather: Option<crate::world::weather::GameWeather>,
}

// Functions for Save Data
impl SaveData {
    // Create a new Save Data
    fn new() -> Self {
        Self {
            player: None,
            time: None,
            weather: None,
        }
    }
}
