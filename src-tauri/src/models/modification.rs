use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

pub const MODS: &str = "mods";

#[derive(Serialize, Deserialize, Debug)]
pub struct Modification {
    id: Thing,
    pub install_url: Cow<'static, str>,
    pub name: Cow<'static, str>,
    pub version: Cow<'static, str>,
}
