use crate::{db_manager::DbManager, models::modification::Modification};
use std::{io::Cursor, path::Path};

const MOD_DIR: &str = "mods";

#[tauri::command]
pub async fn get_modifications(
    db: tauri::State<'_, DbManager>,
) -> Result<Vec<Modification>, String> {
    db.get_modifications().await
}

#[tauri::command]
pub async fn edit_modification(
    old_name: String,
    new_name: String,
    version: String,
    db_manager: tauri::State<'_, DbManager>,
) -> Result<(), String> {
    db_manager
        .edit_modification(old_name, new_name, version)
        .await
}

#[tauri::command]
pub async fn delete_modification(
    modification: Modification,
    db_manager: tauri::State<'_, DbManager>,
) -> Result<(), String> {
    let profiles_related_to_modification = db_manager
        .get_related_profiles(&modification)
        .await
        .map_err(|err| err.to_string())?;

    for profile in profiles_related_to_modification {
        profile
            .delete(&db_manager.db)
            .await
            .map_err(|err| err.to_string())?;
    }

    db_manager
        .delete_modification(&modification)
        .await
        .map_err(|_err| "Failed to delete Mod".to_string())?;

    modification
        .delete(&db_manager.db)
        .await
        .map_err(|err| err.to_string())?;

    Ok(())
}

async fn download_file_at_url(url: &str) -> Result<Vec<u8>, String> {
    let response = reqwest::get(url).await.map_err(|err| {
        let err = err.without_url();
        if err.is_builder() {
            "Invalid URL".into()
        } else if err.is_connect() {
            "Connection Error".into()
        } else if err.is_timeout() {
            "Connection timed out".into()
        } else {
            format!("Error [{err}]")
        }
    })?;

    response
        .bytes()
        .await
        .map(|bytes| bytes.to_vec())
        .map_err(|err| {
            let err = err.without_url();
            if err.is_builder() {
                "Invalid URL".into()
            } else if err.is_connect() {
                "Connection Error".into()
            } else if err.is_timeout() {
                "Connection timed out".into()
            } else {
                format!("Error [{err}]")
            }
        })
}

#[tauri::command]
pub async fn add_modification(
    name: String,
    version: String,
    download_url: String,
    app_handle: tauri::AppHandle,
    db_manager: tauri::State<'_, DbManager>,
) -> Result<(), String> {
    let local_app_data = app_handle
        .path_resolver()
        .app_local_data_dir()
        .ok_or(String::from("Couldn't find Local App Data"))?;

    let modification_install_path = Path::new(&local_app_data).join(MOD_DIR).join(&name);

    let maybe_zip = download_file_at_url(&download_url).await?;
    let mut archive = zip::ZipArchive::new(Cursor::new(maybe_zip))
        .map_err(|_err| "Downloaded file is not a ZIP file".to_string())?;

    archive
        .extract(&modification_install_path)
        .map_err(|_err| "Unable to extract the Zip file".to_string())?;

    let modification = Modification::new(
        name,
        version,
        download_url,
        modification_install_path.display().to_string(),
    );

    match db_manager.create_modification(modification).await {
        Ok(t) => Ok(t),
        Err(why) => {
            tokio::fs::remove_dir_all(modification_install_path)
                .await
                .map_err(|err| err.to_string())?;
            Err(why)
        }
    }
}
