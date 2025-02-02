use std::collections::{HashMap, HashSet};
use shipyard::{Component, Unique, EntityId};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize, Deserializer};

use crate::{
    bitflags_with_str,
    enum_with_str,
    components::{
        character::{Race, Sensei}
    }
};

enum_with_str! {
    #[derive(Component)]
    pub enum ItemType : u8 {
        UNKNOWN = 0, // Unknown item type
        LIGHT = 1, // Item is a light source
        SCROLL = 2, // Item is a scroll
        WAND = 3, // Item is a wand
        STAFF = 4, // Item is a staff
        WEAPON = 5, // Item is a weapon
        FIREWEAPON = 6, // Unimplemented
        CAMPFIRE = 7, // Burn things for fun!
        TREASURE = 8, // Item is a treasure, not gold
        ARMOR = 9, // Item is armor
        POTION = 10, // Item is a potion
        WORN = 11, // Unimplemented
        OTHER = 12, // Misc object
        TRASH = 13, // Trash - shopkeeps won't buy
        TRAP = 14, // Unimplemented
        CONTAINER = 15, // Item is a container
        NOTE = 16, // Item is note
        DRINKCON = 17, // Item is a drink container
        KEY = 18, // Item is a key
        FOOD = 19, // Item is food
        MONEY = 20, // Item is money (gold)
        PEN = 21, // Item is a pen
        BOAT = 22, // Item is a boat
        FOUNTAIN = 23, // Item is a fountain
        VEHICLE = 24, // Item is a vehicle
        HATCH = 25, // Item is a vehicle hatch
        WINDOW = 26, // Item is a vehicle window
        CONTROL = 27, // Item is a vehicle control
        PORTAL = 28, // Item is a portal
        SPELLBOOK = 29, // Item is a spellbook
        BOARD = 30, // Item is a message board
        CHAIR = 31, // Is a chair
        BED = 32, // Is a bed
        YUM = 33, // This was good food
        PLANT = 34, // This will grow!
        FISHPOLE = 35, // FOR FISHING
        FISHBAIT = 36, // DITTO
    }
}

impl Default for ItemType {
    fn default() -> Self {
        ItemType::UNKNOWN
    }
}

enum_with_str! {
    pub enum LiquidType: u8 {
        WATER = 0, // Water
        BEER = 1, // Beer
        WINE = 2, // Wine
        ALE = 3, // Ale
        DARKALE = 4, // Dark Ale
        WHISKY = 5, // Whisky
        LEMONADE = 6, // Lemonade
        FIREBRT = 7, // Firebreather
        LOCALSPC = 8, // Local Specialty
        SLIME = 9, // Slime
        MILK = 10, // Milk
        TEA = 11, // Tea
        COFFEE = 12, // Coffee
        BLOOD = 13, // Blood
        SALTWATER = 14, // Saltwater
        CLEARWATER = 15, // Clear Water
    }
}

enum_with_str! {
    pub enum MaterialType: u8 {
        BONE = 0, // Bone
        CERAMIC = 1, // Ceramic
        COPPER = 2, // Copper
        DIAMOND = 3, // Diamond
        GOLD = 4, // Gold
        IRON = 5, // Iron
        LEATHER = 6, // Leather
        MITHRIL = 7, // Mithril
        OBSIDIAN = 8, // Obsidian
        STEEL = 9, // Steel
        STONE = 10, // Stone
        SILVER = 11, // Silver
        WOOD = 12, // Wood
        GLASS = 13, // Glass
        ORGANIC = 14, // Organic
        CURRENCY = 15, // Currency
        PAPER = 16, // Paper
        COTTON = 17, // Cotton
        SATIN = 18, // Satin
        SILK = 19, // Silk
        BURLAP = 20, // Burlap
        VELVET = 21, // Velvet
        PLATINUM = 22, // Platinum
        ADAMANTINE = 23, // Adamantine
        WOOL = 24, // Wool
        ONYX = 25, // Onyx
        IVORY = 26, // Ivory
        BRASS = 27, // Brass
        MARBLE = 28, // Marble
        BRONZE = 29, // Bronze
        KACHIN = 30, // Kachin
        RUBY = 31, // Ruby
        SAPPHIRE = 32, // Sapphire
        EMERALD = 33, // Emerald
        GEMSTONE = 34, // Gemstone
        GRANITE = 35, // Granite
        ENERGY = 36, // Energy
        HEMP = 37, // Hemp
        CRYSTAL = 38, // Crystal
        EARTH = 39, // Earth
        LIQUID = 40, // Liquid
        CLOTH = 41, // Cloth
        METAL = 42, // Metal
        WAX = 43, // Wax
        OTHER = 44, // Other
        FOOD = 45, // Food
        OIL = 46, // Oil
    }
}

bitflags_with_str! {
    #[derive(Component, Default, Debug, Serialize, Deserialize)]
    pub struct WearFlags: u32 {
        const TAKE = 1 << 0;
        const FINGER = 1 << 1;
        const NECK = 1 << 2;
        const BODY = 1 << 3;
        const HEAD = 1 << 4;
        const LEGS = 1 << 5;
        const FEET = 1 << 6;
        const HANDS = 1 << 7;
        const ARMS = 1 << 8;
        const SHIELD = 1 << 9;
        const ABOUT = 1 << 10;
        const WAIST = 1 << 11;
        const WRIST = 1 << 12;
        const WIELD = 1 << 13;
        const HOLD = 1 << 14;
        const PACK = 1 << 15;
        const EAR = 1 << 16;
        const WINGS = 1 << 17;
        const EYE = 1 << 18;
    }
}


bitflags_with_str! {
    #[derive(Component, Default, Debug, Serialize, Deserialize)]
    pub struct ItemFlags: u64 {
        const GLOW = 1 << 0; // Item is glowing
        const HUM = 1 << 1; // Item is humming
        const NORENT = 1 << 2; // Item cannot be rented
        const NODONATE = 1 << 3; // Item cannot be donated
        const NOINVIS = 1 << 4; // Item cannot be made invis
        const INVISIBLE = 1 << 5; // Item is invisible
        const MAGIC = 1 << 6; // Item is magical
        const NODROP = 1 << 7; // Item is cursed: can't drop
        const BLESS = 1 << 8; // Item is blessed
        const NOSELL = 1 << 9; // Shopkeepers won't touch it
        const TWO_HANDED = 1 << 10; // Requires two free hands
        const UNIQUE_SAVE = 1 << 11; // Unique object save
        const BROKEN = 1 << 12; // Item is broken
        const UNBREAKABLE = 1 << 13; // Item is unbreakable
        const DOUBLE = 1 << 14; // Double weapon
        const CARD = 1 << 15; // Item is a card
        const CBOARD = 1 << 16;
        const FORGED = 1 << 17;
        const SHEATH = 1 << 18;
        const BURIED = 1 << 19;
        const SLOT1 = 1 << 20;
        const SLOT2 = 1 << 21;
        const TOKEN = 1 << 22;
        const SLOT_ONE = 1 << 23;
        const SLOTS_FILLED = 1 << 24;
        const RESTRING = 1 << 25;
        const CUSTOM = 1 << 26;
        const PROTECTED = 1 << 27;
        const NORANDOM = 1 << 28;
        const THROW = 1 << 29;
        const HOT = 1 << 30;
        const PURGE = 1 << 31;
        const ICE = 1 << 32;
        const DUPLICATE = 1 << 33;
        const MATURE = 1 << 34;
        const CARDCASE = 1 << 35;
        const NOPICKUP = 1 << 36;
        const NOSTEAL = 1 << 37;
    }
}



#[derive(Component, Clone, Default, Debug, Serialize, Deserialize)]
pub struct OnlySensei(pub HashSet<Sensei>);

#[derive(Component, Clone, Default, Debug, Serialize, Deserialize)]
pub struct AntiSensei(pub HashSet<Sensei>);

#[derive(Component, Clone, Default, Debug, Serialize, Deserialize)]
pub struct OnlyRace(pub HashSet<Race>);

#[derive(Component, Clone, Default, Debug, Serialize, Deserialize)]
pub struct AntiRace(pub HashSet<Race>);

pub struct Cost(pub u64);