use chrono::Utc;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::any::Any,
    opt::Resource,
    sql::{Datetime, Id, Thing},
    Surreal,
};

pub const MOD: &str = "mod";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Modification {
    pub id: Thing,

    pub download_url: String,
    pub name: String,
    pub version: String,

    pub created_at: Datetime,

    pub install_path: String,
}

impl Modification {
    pub fn new(name: String, version: String, download_url: String, install_path: String) -> Self {
        Self {
            id: Thing::from((MOD, Id::rand())),
            download_url,
            name,
            version,
            created_at: Utc::now().into(),
            install_path,
        }
    }

    pub async fn delete(self, db: &Surreal<Any>) -> anyhow::Result<()> {
        tokio::fs::remove_dir_all(self.install_path).await?;
        db.delete(Resource::from(self.id)).await.map(|_| ())?;
        Ok(())
    }
}
