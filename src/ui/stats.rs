use ratatui::text::Line;

// Struct for Stats
pub struct Stats {
    player_id: String,
    player_name: String,
    town_name: String,
    location: String,
    room_id: String,
}

// Functions for Stats
impl Stats {
    // Create new Stats
    pub fn new() -> Self {
        Self {
            player_id: String::new(),
            player_name: String::new(),
            town_name: String::new(),
            location: String::new(),
            room_id: String::new(),
        }
    }

    // Updates any dynamic parts of Stats
    pub fn update(&mut self, managers: &super::display::Managers) {
        match managers.state_manager.current_state {
            // Game, Save Game, Quit Game, Time, Weather, Travel Town, Travel Building, Building,
            // and Room
            crate::core::states::StateType::Game
            | crate::core::states::StateType::GameSaveSuccess
            | crate::core::states::StateType::GameSaveError
            | crate::core::states::StateType::GameQuit
            | crate::core::states::StateType::Time
            | crate::core::states::StateType::Weather
            | crate::core::states::StateType::TravelTown
            | crate::core::states::StateType::TravelBuilding
            | crate::core::states::StateType::Building
            | crate::core::states::StateType::Room => {
                //Get player ID and name
                if let Some(player) = managers.world_manager.player.as_ref() {
                    self.player_id = format!("Player ID: {}", player.id);
                    self.player_name = format!("Player Name: {}", player.name);
                } else {
                    self.player_id = "Failed to initialize player ID.".into();
                    self.player_name = "Failed to initialize player name".into();
                }

                // Get town name
                self.town_name = if let Some(player) = managers.world_manager.player.as_ref() {
                    if let Some(world) = managers.world_manager.world.as_ref() {
                        if let Some(town) = world.towns.get(&player.current_town_id) {
                            format!("Current Town: {}.", town.name)
                        } else {
                            format!(
                                "Failed to find town name for town ID: {}",
                                player.current_town_id
                            )
                        }
                    } else {
                        format!(
                            "Failed to find town name for town ID: {}",
                            player.current_town_id
                        )
                    }
                } else {
                    "Error getting town info!".into()
                };

                // Get location
                self.location = if let Some(player) = managers.world_manager.player.as_ref() {
                    if let Some(current_building_id) = player.current_building_id.as_ref() {
                        if let Some(world) = managers.world_manager.world.as_ref() {
                            if let Some(building) = world.buildings.get(current_building_id) {
                                format!("Location: {}", building.name)
                            } else {
                                format!(
                                    "Failed to find building name for building ID: {}",
                                    current_building_id
                                )
                            }
                        } else {
                            format!(
                                "Failed to find building name for building ID: {}",
                                current_building_id
                            )
                        }
                    } else {
                        "Location: Outside".into()
                    }
                } else {
                    "Error getting location info!".into()
                };

                // Get room ID
                self.room_id = if let Some(player) = managers.world_manager.player.as_ref() {
                    if let Some(room_id) = player.current_room_id.as_ref() {
                        format!("Room: {}", room_id)
                    } else {
                        "Room: None".into()
                    }
                } else {
                    "Error getting room info!".into()
                };
            }
            // All other states
            _ => {}
        }
    }

    // Renders the Stats based on current state
    pub fn render(&self, managers: &super::display::Managers) -> Vec<Line> {
        match managers.state_manager.current_state {
            // Game, Save Game, Quit Game, Time, Weather, Travel Town, Travel Building, Building,
            // and Room
            crate::core::states::StateType::Game
            | crate::core::states::StateType::GameSaveSuccess
            | crate::core::states::StateType::GameSaveError
            | crate::core::states::StateType::GameQuit
            | crate::core::states::StateType::Time
            | crate::core::states::StateType::Weather
            | crate::core::states::StateType::TravelTown
            | crate::core::states::StateType::TravelBuilding
            | crate::core::states::StateType::Building
            | crate::core::states::StateType::Room => {
                vec![
                    Line::from(self.player_id.clone()),
                    Line::from(self.player_name.clone()),
                    Line::from(self.town_name.clone()),
                    Line::from(self.location.clone()),
                    Line::from(self.room_id.clone()),
                ]
            }
            // All other states
            _ => {
                vec![Line::from("")]
            }
        }
    }
}
