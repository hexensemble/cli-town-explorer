// Enum for state types
#[derive(Clone)]
pub enum StateType {
    MainMenu,
    Name,
    NameConfirm,
    Game,
    GameQuit,
    Time,
}

// Struct for state manager
pub struct StateManager {
    pub current_state: StateType,
}

// Functions for state manager
impl StateManager {
    // Create a new state manager, defaults to MainMenu
    pub fn new() -> Self {
        Self {
            current_state: StateType::MainMenu,
        }
    }
}
