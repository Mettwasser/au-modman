use std::path::{Path, PathBuf};

use chrono::Utc;
use surrealdb::sql::{Thing, Value};
use tokio::fs;

use crate::{
    db_manager::DbManager,
    models::{
        config::Config,
        count::Count,
        modification::Modification,
        profile::{CreateProfile, Profile, ProfileWithMods, PROFILE},
    },
};

const PROFILE_DIR: &str = "profiles";

#[async_recursion::async_recursion]
async fn copy_dir(source_dir: &PathBuf, dest_dir: &PathBuf) -> std::io::Result<()> {
    // Create the destination directory if it doesn't exist
    fs::create_dir_all(dest_dir).await?;

    // Read the contents of the source directory
    let mut dir = fs::read_dir(source_dir).await?;

    // Iterate over the entries in the source directory
    while let Some(entry) = dir.next_entry().await? {
        let source_path = entry.path();
        let dest_path = dest_dir.join(entry.file_name());

        // Copy the file or directory to the destination directory
        if entry.file_type().await?.is_dir() {
            copy_dir(&source_path, &dest_path).await?;
        } else {
            fs::copy(&source_path, &dest_path).await?;
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn add_profile(
    profile_name: String,
    modifications: Vec<Modification>,
    app_handle: tauri::AppHandle,
    db_manager: tauri::State<'_, DbManager>,
) -> Result<(), String> {
    let local_app_data = app_handle
        .path_resolver()
        .app_local_data_dir()
        .ok_or(String::from("Couldn't find Local App Data"))?;

    let profile_location = Path::new(&local_app_data)
        .join(PROFILE_DIR)
        .join(&profile_name);

    let mut profile = CreateProfile::new(profile_name, profile_location.display().to_string());

    let au_install_dir = db_manager.get_config("au_install_dir").await.map_err(|_| {
        "Among Us installation path is not set. Set it in the settings Tab.".to_string()
    })?;

    if au_install_dir.is_empty() {
        return Err(
            "Among Us installation path is not set. Set it in the settings Tab.".to_string(),
        );
    }

    copy_dir(
        &PathBuf::from(&au_install_dir),
        &PathBuf::from(&profile_location),
    )
    .await
    .map_err(|err| format!("{err}"))?;

    for modification in modifications {
        profile.modifications.push(Value::Thing(modification.id));

        copy_dir(
            &PathBuf::from(modification.install_path),
            &PathBuf::from(&profile_location),
        )
        .await
        .map_err(|err| format!("{err}"))?;
    }

    db_manager.create_profile(profile).await
}

#[tauri::command]
pub async fn get_profiles(
    db_manager: tauri::State<'_, DbManager>,
) -> Result<Vec<ProfileWithMods>, String> {
    db_manager
        .get_profiles()
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn delete_profile(
    profile: Profile,
    db_manager: tauri::State<'_, DbManager>,
) -> Result<(), String> {
    profile
        .delete(&db_manager.db)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn launch_profile(
    profile: Profile,
    db_manager: tauri::State<'_, DbManager>,
) -> Result<(), String> {
    tokio::process::Command::new(PathBuf::from(profile.folder_location).join("Among Us.exe"))
        .spawn()
        .map_err(|err| err.to_string())?
        .wait()
        .await
        .map_err(|err| err.to_string())?;

    db_manager
        .set_config(Config::new("last_played".into(), Utc::now().to_rfc3339()))
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn edit_profile(
    profile: Thing,
    new_name: String,
    db_manager: tauri::State<'_, DbManager>,
) -> Result<(), String> {
    db_manager.edit_profile(profile, new_name).await
}

#[tauri::command]
pub async fn get_profile_count(db_manager: tauri::State<'_, DbManager>) -> Result<i32, String> {
    let mut result = db_manager
        .db
        .query(format!("SELECT count() FROM {PROFILE}"))
        .await
        .map_err(|err| err.to_string())?;

    let profile_count: Option<Count> = result.take(0).map_err(|err| err.to_string())?;

    Ok(profile_count.map(|c| c.count).unwrap_or(0))
}
