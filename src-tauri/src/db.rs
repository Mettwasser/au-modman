use std::{env::current_exe, path::Path};

use surrealdb::{
    engine::any::{connect, Any},
    Surreal,
};

pub const DB_NAME: &str = "au_modman.db";

fn get_db_file() -> anyhow::Result<String> {
    let exe_path = current_exe()?;
    let exe_cwd = Path::new(&exe_path).parent().unwrap();

    let db_filepath = &Path::join(exe_cwd, DB_NAME);

    Ok(db_filepath.to_str().unwrap().to_string())
}

pub async fn get_db() -> anyhow::Result<Surreal<Any>> {
    let db_path = get_db_file()?;

    let db = connect(format!("file://{}", db_path)).await?;

    db.use_ns("main").use_db("main").await?;

    Ok(db)
}
