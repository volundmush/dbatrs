use std::collections::HashMap;
use shipyard::{Component, Unique, EntityId};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize, Deserializer};

use crate::{bitflags_with_str, enum_with_str};

bitflags_with_str! {
    pub struct ZoneFlags: u128 {

    }
}