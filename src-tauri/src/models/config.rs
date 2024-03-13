use std::borrow::Borrow;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Id, Thing};

pub const CONFIG: &str = "config";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub id: Thing,
    pub value: String,
}

impl Config {
    pub fn new(name: String, value: String) -> Self {
        Self {
            id: (CONFIG, name.borrow()).into(),
            value,
        }
    }
}
