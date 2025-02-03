use std::collections::HashMap;
use shipyard::{Component, Unique, EntityId};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize, Deserializer};

use crate::{bitflags_with_str, enum_with_str};

bitflags_with_str! {
    #[derive(Component)]
    pub struct ZoneFlags: u128 {
        const CLOSED = 1 << 0;
        const NOIMMORT = 1 << 1;
        const QUEST = 1 << 2;
        const DBALLS = 1 << 3;
    }
}