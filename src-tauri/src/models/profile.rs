use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use surrealdb::sql::{Array, Datetime, Thing};

use super::modification::Modification;

pub const PROFILE: &str = "profile";

// TODO: https://docs.rs/surrealdb/latest/surrealdb/
// Implement fully

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    id: Thing,
    pub changed_at: Datetime,
    pub created_at: Datetime,
    pub folder_location: String,
    pub modification: Modification, // Nested struct
    pub name: Cow<'static, str>,
}
