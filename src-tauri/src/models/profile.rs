use std::borrow::{Borrow, Cow};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::Datetime;

use super::{modification::Modification, GetThing};

pub const PROFILE: &str = "profile";

// TODO: https://docs.rs/surrealdb/latest/surrealdb/
// Implement fully

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile<'a, 'b> {
    pub name: Cow<'a, str>,
    pub folder_location: Cow<'a, str>,
    pub created_at: Datetime,
    pub changed_at: Option<Datetime>,
    pub modifications: Vec<Modification<'b>>,
}

impl<'a, 'b> Profile<'a, 'b> {
    pub fn new(name: Cow<'a, str>, folder_location: Cow<'a, str>) -> Self {
        Self {
            changed_at: None,
            created_at: Utc::now(),
            modifications: Vec::new(),
            folder_location,
            name,
        }
    }
}

impl<'a, 'b> GetThing for Profile<'a, 'b> {
    fn get_thing(&self) -> Thing {
        ("profile", self.name.borrow()).into()
    }
}
