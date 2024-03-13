use chrono::Utc;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::any::Any,
    opt::Resource,
    sql::{Array, Datetime, Id, Thing},
    Surreal,
};

use super::modification::Modification;

pub const PROFILE: &str = "profile";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateProfile {
    pub id: Thing,
    pub name: String,
    pub folder_location: String,

    pub created_at: Datetime,

    pub modifications: Array,
}

impl CreateProfile {
    pub fn new(name: String, folder_location: String) -> Self {
        Self {
            id: Thing::from((PROFILE, Id::rand())),
            created_at: Utc::now().into(),
            modifications: Array::new(),
            folder_location,
            name,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Profile {
    pub id: Thing,
    pub name: String,
    pub folder_location: String,

    pub created_at: Datetime,

    pub modifications: Vec<Thing>,
}

impl Profile {
    pub async fn delete(self, db: &Surreal<Any>) -> anyhow::Result<()> {
        tokio::fs::remove_dir_all(self.folder_location).await?;
        db.delete(Resource::from(self.id)).await.map(|_| ())?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProfileWithMods {
    pub id: Thing,
    pub name: String,
    pub folder_location: String,

    pub created_at: Datetime,

    pub modifications: Vec<Modification>,
}
