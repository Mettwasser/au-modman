use std::borrow::{Borrow, Cow};

use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};

pub const MODS: &str = "mods";

#[derive(Serialize, Deserialize, Debug)]
pub struct Modification<'a> {
    pub id: Thing,
    pub download_url: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub version: Cow<'a, str>,
}

impl<'a> Modification<'a> {
    pub fn new(name: Cow<'a, str>, version: Cow<'a, str>, download_url: Cow<'a, str>) -> Self {
        Self {
            id: Thing::from((MODS, Id::from(vec![name.borrow(), version.borrow()]))),
            download_url,
            name,
            version,
        }
    }
}
