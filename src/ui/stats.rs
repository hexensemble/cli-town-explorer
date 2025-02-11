use ratatui::text::Line;

// Struct for stats
pub struct Stats {}

// Functions for stats
impl Stats {
    // Create new stats
    pub fn new() -> Self {
        Self {}
    }

    // Update stats based on current state
    pub fn update(&self, _state_manager: &crate::core::states::StateManager) {}

    // Render stats based on current state
    pub fn render(
        &self,
        state_manager: &crate::core::states::StateManager,
        world_manager: &crate::world::manager::WorldManager,
    ) -> Vec<Line> {
        match state_manager.current_state {
            // Game, Quit Game - Confirm, and Time
            crate::core::states::StateType::Game
            | crate::core::states::StateType::GameQuit
            | crate::core::states::StateType::Time => {
                if let Some(player) = &world_manager.player {
                    let player_id = format!("Player ID: {}", player.id);
                    let player_name = format!("Player Name: {}", player.name);
                    vec![Line::from(player_id), Line::from(player_name)]
                } else {
                    vec![Line::from("Player not initialized")]
                }
            }
            // All other states
            _ => {
                vec![Line::from("")]
            }
        }
    }
}
