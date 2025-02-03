pub struct State {
    pub state_type: StateType,
    pub last_state: StateType,
}

impl State {
    pub fn new() -> Self {
        Self {
            state_type: StateType::MainMenu,
            last_state: StateType::MainMenu,
        }
    }
}

#[derive(Clone)]
pub enum StateType {
    MainMenu,
    NewGame,
}
