// Enum for state types
#[derive(Clone)]
pub enum StateType {
    MainMenu,
    NewGame,
}

// Struct for state manager
pub struct StateManager {
    pub current_state: StateType,
    pub last_state: StateType,
}

// Functions for state manager
impl StateManager {
    // Create a new state manager, defaults to MainMenu
    pub fn new() -> Self {
        Self {
            current_state: StateType::MainMenu,
            last_state: StateType::MainMenu,
        }
    }
}
