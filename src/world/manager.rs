// Struct for World Manager
pub struct WorldManager {
    pub player: Option<crate::entities::player::Player>,
}

// Functions for World Manager
impl WorldManager {
    // Create a new World Manager
    pub fn new() -> Self {
        Self { player: None }
    }
}
