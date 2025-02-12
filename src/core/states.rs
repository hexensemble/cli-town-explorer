// Enum for state types
#[derive(Clone)]
pub enum StateType {
    MainMenu,
    Name,
    NameConfirm,
    Game,
    GameQuit,
    Time,
    Weather,
}

// Struct for State Manager
pub struct StateManager {
    pub current_state: StateType,
}

// Functions for State Manager
impl StateManager {
    // Create a new State Manager, defaults to Main Menu
    pub fn new() -> Self {
        Self {
            current_state: StateType::MainMenu,
        }
    }
}
