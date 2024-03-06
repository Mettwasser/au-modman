use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

pub const MODS: &str = "mods";

#[derive(Serialize, Deserialize, Debug)]
pub struct Modification<'a> {
    pub download_url: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub version: Cow<'a, str>,
}

impl<'a> Modification<'a> {
    pub fn new(name: Cow<'a, str>, version: Cow<'a, str>, download_url: Cow<'a, str>) -> Self {
        Self {
            download_url,
            name,
            version,
        }
    }
}
