use std::collections::HashMap;
use shipyard::{Component, Unique, EntityId};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize, Deserializer};

use crate::{bitflags_with_str, enum_with_str};

enum_with_str! {
    #[derive(Component, Hash)]
    pub enum Race: u8 {
        Spirit = 0, // Spirit
        Human = 1, // Human
        Saiyan = 2, // Saiyan
        Icer = 3, // Icer
        Konatsu = 4, // Konatsu
        Namekian = 5, // Namekian
        Mutant = 6, // Mutant
        Kanassan = 7, // Kanassan
        Halfbreed = 8, // Halfbreed
        BioAndroid = 9, // BioAndroid
        Android = 10, // Android
        Demon = 11, // Demon
        Majin = 12, // Majin
        Kai = 13, // Kai
        Tuffle = 14, // Tuffle
        Hoshijin = 15, // Hoshijin
        Animal = 16, // Animal
        Saiba = 17, // Saiba
        Serpent = 18, // Serpent
        Ogre = 19, // Ogre
        Yardratian = 20, // Yardratian
        Arlian = 21, // Arlian
        Dragon = 22, // Dragon
        Mechanical = 23, // Mechanical
    }
}

enum_with_str! {
    #[derive(Component, Hash)]
    pub enum Sensei: u8 {
        Commoner = 0, // Commoner
        Roshi = 1, // Roshi
        Piccolo = 2, // Piccolo
        Krane = 3, // Krane
        Nail = 4, // Nail
        Bardock = 5, // Bardock
        Ginyu = 6, // Ginyu
        Frieza = 7, // Frieza
        Tapion = 8, // Tapion
        Sixteen = 9, // Sixteen
        Dabura = 10, // Dabura
        Kibito = 11, // Kibito
        Jinto = 12, // Jinto
        Tsuna = 13, // Tsuna
        Kurzak = 14, // Kurzak
    }
}

enum_with_str! {
    #[derive(Component)]
    pub enum Sex : u8 {
        Neuter = 0,
        Male = 1,
        Female = 2
    }
}