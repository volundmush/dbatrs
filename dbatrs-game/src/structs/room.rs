use std::collections::HashMap;
use shipyard::{Component, Unique, EntityId};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize, Deserializer};

use crate::{bitflags_with_str, enum_with_str};

enum_with_str! {
    #[derive(Hash)]
    pub enum Direction: u8 {
        NORTH = 0,
        EAST = 1,
        SOUTH = 2,
        WEST = 3,
        UP = 4,
        DOWN = 5,
        NORTHWEST = 6,
        NORTHEAST = 7,
        SOUTHEAST = 8,
        SOUTHWEST = 9,
        IN = 10,
        OUT = 11
    }
}

bitflags_with_str! {
    #[derive(Component)]
    pub struct RoomFlags: u128 {
        const DARK = 1 << 0; // Dark
        const DEATH = 1 << 1; // Death trap
        const NOMOB = 1 << 2; // MOBs not allowed
        const INDOORS = 1 << 3; // Indoors
        const PEACEFUL = 1 << 4; // Violence not allowed
        const SOUNDPROOF = 1 << 5; // Shouts, gossip blocked
        const NOTRACK = 1 << 6; // Track won't go through
        const NOINSTANT = 1 << 7; // IT not allowed
        const TUNNEL = 1 << 8; // Room for only 1 person
        const PRIVATE = 1 << 9; // Can't teleport in
        const GODROOM = 1 << 10; // LVL_GOD+ only allowed
        const HOUSE = 1 << 11; // Room is a house
        const HOUSE_CRASH = 1 << 12; // House needs saving
        const ATRIUM = 1 << 13; // The door to a house
        const OLC = 1 << 14; // Modifiable/!compress
        const BFS_MARK = 1 << 15; // Breath-first search mark
        const VEHICLE = 1 << 16; // Requires a vehicle to pass
        const UNDERGROUND = 1 << 17; // Room is below ground
        const CURRENT = 1 << 18; // Room moves with random currents
        const TIMED_DT = 1 << 19; // Room has a timed death trap
        const EARTH = 1 << 20; // Room is on Earth
        const VEGETA = 1 << 21; // Room is on Vegeta
        const FRIGID = 1 << 22; // Room is on Frigid
        const KONACK = 1 << 23; // Room is on Konack
        const NAMEK = 1 << 24; // Room is on Namek
        const NEO = 1 << 25; // Room is on Neo
        const AL = 1 << 26; // Room is on AL
        const SPACE = 1 << 27; // Room is in Space
        const HELL = 1 << 28; // Room is Punishment Hell
        const REGEN = 1 << 29; // Better regen
        const RHELL = 1 << 30; // Room is HELLLLLLL
        const GRAVITYX10 = 1 << 31; // For rooms that have 10x gravity
        const AETHER = 1 << 32; // Room is on Aether
        const HBTC = 1 << 33; // Room is extra special training area
        const PAST = 1 << 34; // Inside the pendulum room
        const CBANK = 1 << 35; // This room is a clan bank
        const SHIP = 1 << 36; // This room is a private ship room
        const YARDRAT = 1 << 37; // This room is on planet Yardrat
        const KANASSA = 1 << 38; // This room is on planet Kanassa
        const ARLIA = 1 << 39; // This room is on planet Arlia
        const AURA = 1 << 40; // This room has an aura around it
        const EORBIT = 1 << 41; // Earth Orbit
        const FORBIT = 1 << 42; // Frigid Orbit
        const KORBIT = 1 << 43; // Konack Orbit
        const NORBIT = 1 << 44; // Namek Orbit
        const VORBIT = 1 << 45; // Vegeta Orbit
        const AORBIT = 1 << 46; // Aether Orbit
        const YORBIT = 1 << 47; // Yardrat Orbit
        const KANORB = 1 << 48; // Kanassa Orbit
        const ARLORB = 1 << 49; // Arlia Orbit
        const NEBULA = 1 << 50; // Nebulae
        const ASTERO = 1 << 51; // Asteroid
        const WORMHO = 1 << 52; // Wormhole
        const STATION = 1 << 53; // Space Station
        const STAR = 1 << 54; // Is a star
        const CERRIA = 1 << 55; // This room is on planet Cerria
        const CORBIT = 1 << 56; // This room is in Cerria's Orbit
        const BEDROOM = 1 << 57; // +25% regen
        const WORKOUT = 1 << 58; // Workout Room
        const GARDEN1 = 1 << 59; // 8 plant garden
        const GARDEN2 = 1 << 60; // 20 plant garden
        const FERTILE1 = 1 << 61;
        const FERTILE2 = 1 << 62;
        const FISHING = 1 << 63;
        const FISHFRESH = 1 << 64;
        const CANREMODEL = 1 << 65;
        const ZENITH = 1 << 66;
        const SAVE = 1 << 67; // Room saves contents
        const ZORBIT = 1 << 68; // Zenith Orbit
    }
}

bitflags_with_str! {
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct ExitFlags: u8 {
        const ISDOOR = 1 << 0;
        const CLOSED = 1 << 1;
        const LOCKED = 1 << 2;
        const PICKPROOF = 1 << 3;
        const SECRET = 1 << 4;
    }
}

#[derive(Component, Clone, Debug, Serialize, Deserialize, Default)]
pub struct Exit {
    pub description: String,
    pub keyword: String,
    pub destination: usize,
    pub key: Option<usize>,
    pub flags: ExitFlags
}

enum_with_str! {
    #[derive(Component)]
    pub enum SectorType: u8 {
        INSIDE = 0,
        CITY = 1,
        FIELD = 2,
        FOREST = 3,
        HILLS = 4,
        MOUNTAIN = 5,
        WATER_SWIM = 6,
        WATER_NOSWIM = 7,
        FLYING = 8,
        UNDERWATER = 9,
        SHOP = 10,
        IMPORTANT = 11,
        DESERT = 12,
        SPACE = 13,
        LAVA = 14
    }
}

impl Default for SectorType {
    fn default() -> Self {
        SectorType::INSIDE
    }
}

#[derive(Component, Clone, Debug, Serialize, Deserialize, Default)]
pub struct Exits {
    pub exits: HashMap<Direction, Exit>
}

#[derive(Component, Clone, Debug, Default)]
pub struct People(pub Vec<EntityId>);