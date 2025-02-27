use petgraph::algo::dijkstra;
use petgraph::graph::NodeIndex;
use petgraph::{Graph, Undirected};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

// Struct for World Manager
pub struct WorldManager {
    pub player: Option<crate::entities::player::Player>,
    pub world: Option<World>,
    world_graph: Option<Graph<String, JourneyInfo, Undirected>>,
    node_indices: HashMap<String, NodeIndex>,
}

// Functions for World Manager
impl WorldManager {
    // Create a new World Manager
    pub fn new() -> Self {
        Self {
            player: None,
            world: None,
            world_graph: None,
            node_indices: HashMap::new(),
        }
    }

    // Clear all data from the World Manager
    pub fn clear(&mut self) {
        self.player = None;
        self.world = None;
        self.world_graph = None;
        self.node_indices = HashMap::new();
    }

    // Load in world JSON and DOT files
    pub fn load_world(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let world_path = PathBuf::from("assets").join("world.json");
        let world_data = fs::read_to_string(world_path)?;
        let world: World = serde_json::from_str(&world_data)?;
        self.world = Some(world);

        let world_graph_path = PathBuf::from("assets").join("world.dot");
        let world_graph_data = fs::read_to_string(world_graph_path)?;
        self.world_graph = Some(Graph::new_undirected());

        match self.world_graph.as_mut() {
            Some(world_graph) => {
                for line in world_graph_data.lines() {
                    if let Some((source, target, label)) = parse_edge_line(line) {
                        let src_index = *self
                            .node_indices
                            .entry(source.clone())
                            .or_insert_with(|| world_graph.add_node(source.clone()));
                        let tgt_index = *self
                            .node_indices
                            .entry(target.clone())
                            .or_insert_with(|| world_graph.add_node(target.clone()));

                        if let Some(journey_info) = JourneyInfo::from_label(&label) {
                            world_graph.add_edge(src_index, tgt_index, journey_info);
                        }
                    }
                }
            }
            None => return Err("Failed to create world graph.".into()),
        }

        Ok(())
    }

    pub fn get_travel_time(&self, origin: &String, destination: &String) -> u32 {
        if let (Some(&origin_unwrapped), Some(&destination_unwrapped)) = (
            self.node_indices.get(origin),
            self.node_indices.get(destination),
        ) {
            if let Some(graph) = self.world_graph.as_ref() {
                let path = dijkstra(graph, origin_unwrapped, Some(destination_unwrapped), |e| {
                    e.weight().distance
                });

                if let Some(cost) = path.get(&destination_unwrapped) {
                    cost * 10 // 1 mile = 10 ticks
                } else {
                    0
                }
            } else {
                0
            }
        } else {
            0
        }
    }
}

// Parses an edge line from the DOT file and extracts (town1, town2, label).
fn parse_edge_line(line: &str) -> Option<(String, String, String)> {
    let line = line.trim();

    if line.starts_with('"') && line.contains("--") && line.contains("[label=") {
        let parts: Vec<&str> = line.split("--").collect();
        if parts.len() == 2 {
            // Trim and remove quotes from the source town
            let source = parts[0].trim().trim_matches('"').to_string();

            let target_and_label = parts[1].trim();

            // Find where the target ends (right before the `[`)
            let target_end = target_and_label.find('[')?;
            let target = target_and_label[..target_end]
                .trim()
                .trim_matches('"')
                .to_string();

            // Extract the label content within quotes
            let label_start = target_and_label.find("label=\"")? + 7;
            let label_end = target_and_label[label_start..].find('"')? + label_start;
            let label = target_and_label[label_start..label_end].trim().to_string();

            return Some((source, target, label));
        }
    }
    None
}

// Struct for storing distance and cost between towns
struct JourneyInfo {
    distance: u32,
    cost: u32,
}

// Functions for JourneyInfo
impl JourneyInfo {
    fn from_label(label: &str) -> Option<Self> {
        let parts: Vec<&str> = label.split("/").map(|s| s.trim()).collect();
        if parts.len() == 2 {
            let distance = parts[0].split_whitespace().next()?.parse().ok()?;
            let cost = parts[1].split_whitespace().next()?.parse().ok()?;
            Some(Self { distance, cost })
        } else {
            None
        }
    }
}

// Struct for representing a world. Contains global lists
#[derive(Serialize, Deserialize, Debug)]
pub struct World {
    pub towns: HashMap<u32, Town>,
    buildings: HashMap<u32, Building>,
    rooms: HashMap<u32, Room>,
    npcs: HashMap<u32, Npc>,
    containers: HashMap<u32, Container>,
}

// Struct for representing a town
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Town {
    id: u32,
    pub name: String,
    coords: (u32, u32),
    number_of_buildings: u32,
    buildings: Vec<Building>,
}

// Struct for representing a building
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Building {
    id: u32,
    name: String,
    building_type: BuildingType,
    town_id: u32,
    coords: (u32, u32),
    rooms: Vec<Room>,
}

// Enum for building types
#[derive(Serialize, Deserialize, Debug, Clone)]
enum BuildingType {
    Residence,
    Shop,
    Tavern,
    Temple,
}

// Struct for representing a room
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Room {
    id: u32,
    town_id: u32,
    building_id: u32,
    npcs: Vec<Npc>,
    containers: Vec<Container>,
}

// Struct for representing an NPC
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Npc {
    id: u32,
    name: String,
    sex: NpcSex,
    race: NpcRace,
    town_id: u32,
    building_id: u32,
    room_id: Option<u32>,
}

// Enum for NPC sex
#[derive(Serialize, Deserialize, Debug, Clone)]
enum NpcSex {
    Male,
    Female,
    Unisex,
}

// Enum for NPC race
#[derive(Serialize, Deserialize, Debug, Clone)]
enum NpcRace {
    Human,
    Elf,
}

// Struct for representing a container
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Container {
    id: u32,
    container_type: ContainerType,
    town_id: u32,
    building_id: u32,
    room_id: u32,
}

// Enum for container types
#[derive(Serialize, Deserialize, Debug, Clone)]
enum ContainerType {
    Barrel,
    Crate,
    Chest,
}
