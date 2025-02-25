use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

// Struct for World Manager
pub struct WorldManager {
    pub player: Option<crate::entities::player::Player>,
    pub world: Option<World>,
}

// Functions for World Manager
impl WorldManager {
    // Create a new World Manager
    pub fn new() -> Self {
        Self {
            player: None,
            world: None,
        }
    }

    pub fn load_world(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = PathBuf::from("assets").join("world.json");
        let data = fs::read_to_string(file_path)?;

        let world: World = serde_json::from_str(&data)?;

        self.world = Some(world);

        Ok(())
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
