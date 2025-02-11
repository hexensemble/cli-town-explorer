// Struct for world manager
pub struct WorldManager {
    pub player: Option<crate::entities::player::Player>,
}

// Functions for world manager
impl WorldManager {
    // Create a new world manager
    pub fn new() -> Self {
        Self { player: None }
    }
}
