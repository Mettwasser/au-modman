use crate::{db_manager::DbManager, models::config::Config};

#[tauri::command]
pub async fn get_config(
    config_name: String,
    db: tauri::State<'_, DbManager>,
) -> Result<String, String> {
    db.get_config(&config_name).await
}

#[tauri::command]
pub async fn set_config(
    config_name: String,
    value: String,
    db: tauri::State<'_, DbManager>,
) -> Result<(), String> {
    db.set_config(Config::new(config_name, value)).await
}
