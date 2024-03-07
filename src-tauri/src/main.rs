// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
mod db_manager;
mod models;

use chrono::{DateTime, Utc};
// use commands::*;
use db_manager::DbManager;
use tauri::Manager;

use std::error::Error;

pub type Datetime = DateTime<Utc>;

fn main() -> Result<(), Box<dyn Error>> {
    tauri::Builder::default()
        .setup(|app| {
            let db = tauri::async_runtime::block_on(DbManager::try_new(app.path_resolver()))?;
            app.manage(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
