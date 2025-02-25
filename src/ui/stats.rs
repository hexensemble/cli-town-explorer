use ratatui::text::Line;

// Struct for Stats
pub struct Stats {}

// Functions for Stats
impl Stats {
    // Create new Stats
    pub fn new() -> Self {
        Self {}
    }

    // Renders the Stats based on current state
    pub fn render(&self, managers: &super::display::Managers) -> Vec<Line> {
        match managers.state_manager.current_state {
            // Game, Save Game, Quit Game, Time, Weather, and Travel
            crate::core::states::StateType::Game
            | crate::core::states::StateType::GameSaveSuccess
            | crate::core::states::StateType::GameSaveError
            | crate::core::states::StateType::GameQuit
            | crate::core::states::StateType::Time
            | crate::core::states::StateType::Weather
            | crate::core::states::StateType::Travel => {
                if let Some(player) = &managers.world_manager.player {
                    let player_id = format!("Player ID: {}", player.id);
                    let player_name = format!("Player Name: {}", player.name);
                    let town_name = format!("Current Town: {}", player.town_name);
                    vec![
                        Line::from(player_id),
                        Line::from(player_name),
                        Line::from(town_name),
                    ]
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
