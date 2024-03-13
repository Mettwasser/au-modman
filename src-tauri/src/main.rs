// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
mod db_manager;
mod models;

use chrono::Utc;
use commands::*;
use db_manager::DbManager;
use tauri::Manager;

use std::error::Error;

pub type DateTime = chrono::DateTime<Utc>;

fn main() -> Result<(), Box<dyn Error>> {
    tauri::Builder::default()
        .setup(|app| {
            let db = tauri::async_runtime::block_on(DbManager::try_new(
                app.path_resolver(),
                app.package_info().version.to_string(),
            ))?;

            app.manage(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_modification,
            get_modifications,
            delete_modification,
            edit_modification,
            get_config,
            set_config,
            add_profile,
            get_profiles,
            delete_profile,
            launch_profile,
            edit_profile
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
