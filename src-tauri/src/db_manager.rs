use surrealdb::{
    engine::any::{connect, Any},
    Surreal,
};
use tauri::PathResolver;

pub const DB_NAME: &str = "au_modman.db";

fn get_db_file_path(pr: PathResolver) -> anyhow::Result<String> {
    let local_app_data = pr.app_local_data_dir().unwrap();
    println!("{local_app_data:?}");

    let db_filepath = local_app_data.join(DB_NAME);
    Ok(db_filepath.to_str().unwrap().to_string())
}

pub struct DbManager {
    pub db: Surreal<Any>,
}

impl DbManager {
    pub async fn try_new(path_resolver: PathResolver) -> anyhow::Result<Self> {
        let db_path = get_db_file_path(path_resolver)?;

        let db = connect(format!("file://{}", db_path)).await?;

        db.use_ns("prod").use_db("main").await?;

        Ok(Self { db })
    }
}
