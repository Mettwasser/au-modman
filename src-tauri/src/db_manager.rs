use crate::models::{
    config::{Config, CONFIG},
    modification::{Modification, MOD},
    profile::{CreateProfile, Profile, ProfileWithMods, PROFILE},
};
use std::{collections::HashMap, path::PathBuf};
use surrealdb::{
    engine::any::{connect, Any},
    opt::Resource,
    sql::Thing,
    Error, Surreal,
};
use tauri::PathResolver;

pub const DB_NAME: &str = "au_modman.db";
const DEFINE_TABLES_SQL: &str = include_str!("../statements/define_tables.surql");

fn get_db_file_path(pr: PathResolver) -> anyhow::Result<String> {
    let local_app_data = pr.app_local_data_dir().unwrap();

    let db_filepath = local_app_data.join(DB_NAME);
    Ok(db_filepath.to_str().unwrap().to_string())
}

/// Tries to check the common Steam & Epic Games Among Us installation dirs and returns either.
fn get_au_install_dir() -> Option<PathBuf> {
    let steam_dir = PathBuf::from(r#"C:\Program Files (x86)\Steam\steamapps\common\Among Us"#);
    let epic_games_dir = PathBuf::from(r#"C:\Program Files\Epic Games\AmongUs"#);

    match (steam_dir.exists(), epic_games_dir.exists()) {
        (true, _) => Some(steam_dir),
        (_, true) => Some(epic_games_dir),
        _ => None,
    }
}

pub struct DbManager {
    pub db: Surreal<Any>,
}

impl DbManager {
    // region: CONSTRUCTOR
    pub async fn try_new(path_resolver: PathResolver, version: String) -> anyhow::Result<Self> {
        let db_path = get_db_file_path(path_resolver)?;

        let db = connect(format!("file://{}", db_path)).await?;

        db.use_ns("prod")
            .use_db(version.split('.').nth(0).unwrap())
            .await?;

        db.query(
            DEFINE_TABLES_SQL
                .replace("@tb_mods", MOD)
                .replace("@tb_profiles", PROFILE),
        )
        .await?;

        let db_manager = Self { db };

        if db_manager.get_config("au_install_dir").await.is_err() {
            db_manager
                .set_config(Config::new(
                    "au_install_dir".into(),
                    get_au_install_dir()
                        .unwrap_or_default()
                        .display()
                        .to_string(),
                ))
                .await
                .unwrap()
        }

        Ok(db_manager)
    }
    // endregion

    // region: MODIFICATION
    pub async fn create_modification(&self, modification: Modification) -> Result<(), String> {
        self.db
            .create(Resource::RecordId(modification.id.clone()))
            .content(&modification)
            .await
            .map_err(|err| match err {
                Error::Db(err) => match err {
                    surrealdb::error::Db::IndexExists {
                        thing: _,
                        index: _,
                        value: _,
                    } => format!(
                        "A Mod with the Name {} and the Version {} already exists",
                        modification.name, modification.version
                    ),
                    _ => "Failed to add the Mod to the Database".to_string(),
                },
                Error::Api(_) => "Failed to add the Mod to the Database".to_string(),
            })?;
        Ok(())
    }

    pub async fn get_modifications(&self) -> Result<Vec<Modification>, String> {
        let mut modifications: Vec<Modification> = self
            .db
            .select(MOD)
            .await
            .map_err(|_err| "Failed to add the Mod to the Database".to_string())?;

        modifications.reverse();

        Ok(modifications)
    }

    pub async fn delete_modification(
        &self,
        modification: &Modification,
    ) -> Result<(), surrealdb::Error> {
        self.db
            .delete(Resource::from(modification.id.clone()))
            .await
            .map(|_| ())
    }

    pub async fn edit_modification(
        &self,
        old_name: String,
        new_name: String,
        version: String,
    ) -> Result<(), String> {
        let result = self
            .db
            .query(format!(
                "UPDATE {MOD} SET name = $new_name WHERE name = $old_name AND version = $version",
            ))
            .bind(HashMap::from([
                ("old_name", &old_name),
                ("new_name", &new_name),
                ("version", &version),
            ]))
            .await
            .map_err(|_err| "Failed to edit Mod".to_string())?;

        result.check().map_err(|err| match err {
            Error::Db(err) => match err {
                surrealdb::error::Db::IndexExists {
                    thing: _,
                    index: _,
                    value: _,
                } => format!(
                    "A Mod with the Name {} and the Version {} already exists",
                    new_name, version
                ),
                _ => "Failed to edit Mod".to_string(),
            },
            Error::Api(_) => "Failed to edit Mod".to_string(),
        })?;

        Ok(())
    }
    // endregion

    // region: CONFIG
    pub async fn get_config(&self, name: &str) -> Result<String, String> {
        let config: Config = self
            .db
            .select((CONFIG, name))
            .await
            .unwrap()
            .ok_or(format!("Couldn't find config under the name {name}"))?;

        Ok(config.value)
    }

    pub async fn set_config(&self, config: Config) -> Result<(), String> {
        self.db
            .update(Resource::RecordId(config.id.clone()))
            .content(config)
            .await
            .map_err(|err| err.to_string())?;

        Ok(())
    }
    // endregion

    // region: PROFILES
    pub async fn create_profile(&self, profile: CreateProfile) -> Result<(), String> {
        self.db
            .create(Resource::RecordId(profile.id.clone()))
            .content(&profile)
            .await
            .map_err(|err| match err {
                Error::Db(err) => match err {
                    surrealdb::error::Db::IndexExists {
                        thing: _,
                        index: _,
                        value: _,
                    } => format!("A Profile with the Name {} already exists", profile.name),
                    _ => "Failed to add the Profile to the Database".to_string(),
                },
                Error::Api(_) => "Failed to add the Profile to the Database".to_string(),
            })?;
        Ok(())
    }

    pub async fn get_profiles(&self) -> Result<Vec<ProfileWithMods>, surrealdb::Error> {
        let mut result = self
            .db
            .query(format!("SELECT * FROM {PROFILE} FETCH modifications"))
            .await?;

        let mut profiles: Vec<ProfileWithMods> = result.take(0)?;

        profiles.reverse();

        Ok(profiles)
    }

    pub async fn edit_profile(&self, profile_id: Thing, new_name: String) -> Result<(), String> {
        let result = self
            .db
            .query("UPDATE $profile SET name = $new_name")
            .bind(("profile", profile_id))
            .bind(("new_name", &new_name))
            .await
            .map_err(|_err| "Failed to edit Profile".to_string())?;

        result.check().map_err(|err| match err {
            Error::Db(err) => match err {
                surrealdb::error::Db::IndexExists {
                    thing: _,
                    index: _,
                    value: _,
                } => format!("A Profile with the Name {} already exists", new_name),
                _ => "Failed to edit Profile".to_string(),
            },
            Error::Api(_) => "Failed to edit Mod".to_string(),
        })?;

        Ok(())
    }

    pub async fn get_related_profiles(
        &self,
        related_to: &Modification,
    ) -> Result<Vec<Profile>, surrealdb::Error> {
        let mut result = self
            .db
            .query(format!(
                "SELECT * FROM {PROFILE} WHERE $id IN modifications"
            ))
            .bind(related_to)
            .await?;

        let profiles: Vec<Profile> = result.take(0)?;

        Ok(profiles)
    }
    // endregion
}
