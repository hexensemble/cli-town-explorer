use serde::{Deserialize, Serialize};

// Struct for player
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: u32,
    pub name: String,
}

// Functions for player
impl Player {
    // Create a new player
    pub fn new(id: u32, name: String) -> Self {
        Self { id, name }
    }
}
