// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
mod db;
mod models;

use chrono::{DateTime, Utc};
use commands::*;
use db::get_db;

use std::error::Error;

pub type Datetime = DateTime<Utc>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db = get_db().await?;
    db.use_ns("prod").use_db("main").await?;

    tauri::Builder::default()
        .manage(db)
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
