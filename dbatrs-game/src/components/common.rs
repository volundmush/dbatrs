use std::collections::HashMap;
use shipyard::{Component, Unique, EntityId};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize, Deserializer};

use crate::{bitflags_with_str, enum_with_str};

// This is a component used for storing all entities which have a long-term presence in the database.
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub enum DbEntityType {
    Zone = 0,
    Room = 1,
    Player = 2,
    NpcProto = 3,
    ItemProto = 4,
    ScriptProto = 5,
    Shop = 6,
    Guild = 7,
    UserAccount = 8
}

// This is a component used for storing the unique identifier of an entity in the database.
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct DbIdentifier {
    pub id: usize,
    pub entity_type: DbEntityType
}

// This unique component indexes all entities by their unique identifier in the database.
#[derive(Unique, Clone, Debug, Default)]
pub struct DbIndexes {
    pub zones: HashMap<usize, EntityId>,
    pub rooms: HashMap<usize, EntityId>,
    pub npc_proto: HashMap<usize, EntityId>,
    pub item_proto: HashMap<usize, EntityId>,
    pub script_proto: HashMap<usize, EntityId>,
    pub shops: HashMap<usize, EntityId>,
    pub guilds: HashMap<usize, EntityId>,
    pub user_accounts: HashMap<usize, EntityId>,
    pub players: HashMap<usize, EntityId>
}

// Each UserAccount uses this for its data storage.
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct UserAccount {
    pub name: String,
    pub password: String,
    pub email: String,
    pub created: DateTime<Utc>,
    pub last_login: DateTime<Utc>,
    pub last_logout: DateTime<Utc>,
    pub last_password_change: DateTime<Utc>,
    pub admin_level: u8,
    pub rpp: usize,
    pub slots: usize,
    pub characters: Vec<usize>
}

// Each PlayerCharacter uses this for its data storage.
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub account: usize
}

// Used for quick identification of physical things in the game world.
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub enum ThingType {
    Room = 0,
    Character = 1,
    Item = 2
}

// Everything except Rooms can have a Location component.
#[derive(Component, Clone, Debug)]
pub enum Location {
    // The EntityId here for Inventory could be an Room, Character, or Item.
    Inventory(EntityId),
    // Though at the moment only Characters can equip stuff.
    Equipped(EntityId, usize),
}

// Items, Characters, and Rooms can all store Items in an inventory.
// We might as well use Equip here too.
// This serves as a kind of reverse lookup for the Location component.
#[derive(Component, Clone, Debug)]
pub struct ThingContents {
    pub items: Vec<EntityId>,
    pub equipment: HashMap<usize, EntityId>
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct ExtraDescriptions {
    pub extras: Vec<(String, String)>
}

// Title is used for anything with an objective name or title.
// IE: Player Character names, Room Names, Item Names, etc.
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Title(pub String);

// Many things have a Description that's shown when looked at.
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Description(pub String);

// Many things have a sequence of Keywords that can be used to refer to them.
// Mostly, this is for Items and NPCs.
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Keywords(pub Vec<String>);

// lastly, shortdescs are used for things that are shown in lists.
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct ShortDesc(pub String);

pub struct Weight(pub f32);


bitflags_with_str! {
    #[derive(Component, Default, Debug, Serialize, Deserialize)]
    pub struct AffectFlags: u64 {
        const DONTUSE = 1 << 0; // DON'T USE!
        const BLIND = 1 << 1; // Char is blind
        const INVISIBLE = 1 << 2; // Char is invisible
        const DETECT_ALIGN = 1 << 3; // Char is sensitive to align
        const DETECT_INVIS = 1 << 4; // Char can see invis chars
        const DETECT_MAGIC = 1 << 5; // Char is sensitive to magic
        const SENSE_LIFE = 1 << 6; // Char can sense hidden life
        const WATERWALK = 1 << 7; // Char can walk on water
        const SANCTUARY = 1 << 8; // Char protected by sanct.
        const GROUP = 1 << 9; // Char is grouped
        const CURSE = 1 << 10; // Char is cursed
        const INFRAVISION = 1 << 11; // Char can see in dark
        const POISON = 1 << 12; // Char is poisoned
        const WEAKENED_STATE = 1 << 13; // Char protected from evil
        const PROTECT_GOOD = 1 << 14; // Char protected from good
        const SLEEP = 1 << 15; // Char magically asleep
        const NOTRACK = 1 << 16; // Char can't be tracked
        const UNDEAD = 1 << 17; // Char is undead
        const PARALYZE = 1 << 18; // Char is paralyzed
        const SNEAK = 1 << 19; // Char can move quietly
        const HIDE = 1 << 20; // Char is hidden
        const UNUSED20 = 1 << 21; // Room for future expansion
        const CHARM = 1 << 22; // Char is charmed
        const FLYING = 1 << 23; // Char is flying
        const WATERBREATH = 1 << 24; // Char can breathe non O2
        const ANGELIC = 1 << 25; // Char is an angelic being
        const ETHEREAL = 1 << 26; // Char is ethereal
        const MAGICONLY = 1 << 27; // Char only hurt by magic
        const NEXTPARTIAL = 1 << 28; // Next action cannot be full
        const NEXTNOACTION = 1 << 29; // Next action cannot attack
        const STUNNED = 1 << 30; // Char is stunned
        const TAMED = 1 << 31; // Char has been tamed
        const CDEATH = 1 << 32; // Char is undergoing creeping death
        const SPIRIT = 1 << 33; // Char has no body
        const STONESKIN = 1 << 34; // Char has temporary DR
        const SUMMONED = 1 << 35; // Char is summoned (i.e. transient)
        const CELESTIAL = 1 << 36; // Char is celestial
        const FIENDISH = 1 << 37; // Char is fiendish
        const FIRE_SHIELD = 1 << 38; // Char has fire shield
        const LOW_LIGHT = 1 << 39; // Char has low light eyes
        const ZANZOKEN = 1 << 40; // Char is ready to zanzoken
        const KNOCKED = 1 << 41; // Char is knocked OUT!
        const MIGHT = 1 << 42; // Strength +3
        const FLEX = 1 << 43; // Agility +3
        const GENIUS = 1 << 44; // Intelligence +3
        const BLESS = 1 << 45; // Bless for better regen
        const BURNT = 1 << 46; // Disintegrated corpse
        const BURNED = 1 << 47; // Burned by honoo or similar skill
        const MBREAK = 1 << 48; // Can't charge while flagged
        const HASS = 1 << 49; // Does double punch damage
        const FUTURE = 1 << 50; // Future Sight
        const PARA = 1 << 51; // Real Paralyze
        const INFUSE = 1 << 52; // Ki infused attacks
        const ENLIGHTEN = 1 << 53; // Enlighten
        const FROZEN = 1 << 54; // They got frozen
        const FIRESHIELD = 1 << 55; // They have a blazing personality
        const ENSNARED = 1 << 56; // They have silk ensnaring their arms!
        const HAYASA = 1 << 57; // They are speedy!
        const PURSUIT = 1 << 58; // Being followed
        const WITHER = 1 << 59; // Their body is withered
        const HYDROZAP = 1 << 60; // Custom Skill Kanso Suru
        const POSITION = 1 << 61; // Better combat position
        const SHOCKED = 1 << 62; // Psychic Shock
        const METAMORPH = 1 << 63; // Metamorphosis
    }
}

enum_with_str! {
    #[derive(Component)]
    pub enum ThingSize : u8 {
        FINE = 0,
        DIMINUTIVE = 1,
        TINY = 2,
        SMALL = 3,
        MEDIUM = 4,
        LARGE = 5,
        HUGE = 6,
        GARGANTUAN = 7,
        COLOSSAL = 8
    }
}

impl Default for ThingSize {
    fn default() -> Self {
        ThingSize::MEDIUM
    }
}