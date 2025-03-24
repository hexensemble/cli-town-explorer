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
            // Game, Save Game, Quit Game, Time, Weather, Travel Town, and Travel Building
            crate::core::states::StateType::Game
            | crate::core::states::StateType::GameSaveSuccess
            | crate::core::states::StateType::GameSaveError
            | crate::core::states::StateType::GameQuit
            | crate::core::states::StateType::Time
            | crate::core::states::StateType::Weather
            | crate::core::states::StateType::TravelTown
            | crate::core::states::StateType::TravelBuilding => {
                if let Some(player) = &managers.world_manager.player {
                    // Get player ID
                    let player_id = format!("Player ID: {}", player.id);

                    // Get player name
                    let player_name = format!("Player Name: {}", player.name);

                    // Get town name
                    let town_name = if let Some(world) = managers.world_manager.world.as_ref() {
                        if let Some(town) = world.towns.get(&player.current_town_id) {
                            format!("Current Town: {}", town.name)
                        } else {
                            format!(
                                "Failed to find town name for town ID: {}",
                                player.current_town_id
                            )
                        }
                    } else {
                        format!(
                            "Failed to find town name for town ID: {}",
                            player.current_town_id
                        )
                    };

                    // Get location
                    let location =
                        if let Some(current_building_id) = player.current_building_id.as_ref() {
                            if let Some(world) = managers.world_manager.world.as_ref() {
                                if let Some(building) = world.buildings.get(current_building_id) {
                                    format!("Location: {}", building.name)
                                } else {
                                    format!(
                                        "Failed to find building name for building ID: {}",
                                        current_building_id
                                    )
                                }
                            } else {
                                format!(
                                    "Failed to find building name for building ID: {}",
                                    current_building_id
                                )
                            }
                        } else {
                            "Location: Outside".into()
                        };

                    // Text to render
                    vec![
                        Line::from(player_id),
                        Line::from(player_name),
                        Line::from(town_name),
                        Line::from(location),
                    ]
                } else {
                    vec![Line::from("Player not initialized.")]
                }
            }
            // All other states
            _ => {
                vec![Line::from("")]
            }
        }
    }
}
