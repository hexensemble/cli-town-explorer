use serde::{Deserialize, Serialize};

// Struct for player
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub current_town_id: u32,
    pub current_building_id: Option<u32>,
    pub current_room_id: Option<u32>,
}

// Functions for player
impl Player {
    // Create a new player
    pub fn new(
        id: u32,
        name: String,
        current_town_id: u32,
        current_building_id: Option<u32>,
        current_room_id: Option<u32>,
    ) -> Self {
        Self {
            id,
            name,
            current_town_id,
            current_building_id,
            current_room_id,
        }
    }
}
