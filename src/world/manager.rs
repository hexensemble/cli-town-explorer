// Struct for world manager
pub struct WorldManager {
    pub players: Vec<crate::entities::player::Player>,
}

// Functions for world manager
impl WorldManager {
    // Create a new world manager
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
        }
    }
}
