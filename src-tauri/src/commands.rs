//! Configuration management and mod listing functionality for the Barotrauma mod manager.
//!
//! This module provides Tauri commands to:
//! - Read and write persistent configuration settings.
//! - List installed mods by analyzing the game directory.
//!
//! The configuration is stored in TOML format under a global config file path,
//! typically in the OS-specific roaming/app data directory.

use configuration::{Config, InstallStrategy};
use std::collections::HashSet;
use std::fs;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::build_info::BuildInfo;
use crate::once::{BARO_MANAGER, STEAM_WORKSHOP_CLIENT, STEAMCMD_MANAGER};
use constants::{BAROTRAUMA_GAME_ID, GLOBAL_CONFIG_FILE, ROAMING};
use fs_extra::dir::CopyOptions;
use futures::TryFutureExt;
use futures::future::try_join_all;
use imagen::{BackgroundConfig, process_background};
use logger::{debug, error, info, warn};
use mod_analyzer::{BarotraumaMod, ModList};
use steam_api::WorkshopItem;

use mod_analyzer::retrieve_mod_metadata as get_mod_metadata;
/// Writes the given configuration to disk in TOML format.
///
/// This function ensures the configuration directory exists (creating it if necessary),
/// then serializes the provided `Config` struct into a pretty-printed TOML string
/// and writes it to the global config file.
///
/// # Parameters
/// - `config`: The `Config` struct to persist to disk.
///
/// # Returns
/// - `Ok(())` if the write operation succeeds.
/// - `Err(String)` with a descriptive error message on failure.
///
/// # Errors
/// This function may fail due to:
/// - Inability to create the configuration directory (`ROAMING`).
/// - Serialization failure when converting the config to TOML.
/// - File I/O errors during writing.
#[tauri::command]
pub fn write_config(config: Config) -> Result<(), String> {
    fs::create_dir_all(ROAMING.clone())
        .map_err(|e| format!("{}, failed to create config directory.", e))?;
    config
        .to_file(GLOBAL_CONFIG_FILE.clone())
        .map_err(|e| format!("{}, failed to write config file.", e))
}

/// Reads the configuration from the global config file.
///
/// Attempts to read and parse the TOML configuration file from the predefined path.
/// If the file does not exist, returns a default configuration instead of an error.
///
/// # Returns
/// - `Ok(Config)` with the parsed configuration on success.
/// - `Ok(Config::default_settings())` if the config file doesn't exist.
/// - `Err(String)` with a descriptive error message if the file exists but cannot be read or parsed.
///
/// # Errors
/// This function may fail due to:
/// - File not being readable (permissions, I/O errors).
/// - Invalid TOML syntax in the config file.
/// - Missing or malformed fields that don't match the `Config` structure.
#[tauri::command]
pub fn read_config() -> Result<Config, String> {
    if GLOBAL_CONFIG_FILE.exists() {
        debug!("Reading config file.");
        Ok(Config::from_file(GLOBAL_CONFIG_FILE.as_path())
            .map_err(|e| format!("{}, failed to parse config file.", e))?)
    } else {
        Ok(Config::default_settings())
    }
}

#[tauri::command]
pub fn get_default_config() -> Config {
    Config::default_settings()
}

/// Lists all installed Barotrauma mods found in the configured game directory.
///
/// This function:
/// 1. Reads the current configuration to get the game installation path.
/// 2. Sets the game directory in the global `BARO_MANAGER`.
/// 3. Refreshes the list of detected mods by scanning the `Mods` folder.
/// 4. Returns a vector of `BarotraumaMod` structs representing each installed mod.
///
/// # Returns
/// - `Ok(Vec<BarotraumaMod>)`: A list of detected mods.
/// - `Err(String)`: If the config cannot be read, the game path is invalid, or mod scanning fails.
///
/// # Errors
/// This function may fail due to:
/// - Failure to read or parse the configuration.
/// - Invalid `game_home` path (e.g., malformed string).
/// - Issues accessing the game's `Mods` directory (permissions, missing folder, etc.).
/// - Failures during mod metadata parsing (e.g., invalid `content.xml`).
///
/// # Note
/// The result is a clone of each mod's data. This is intentional to transfer ownership
/// across the FFI boundary safely (e.g., to JavaScript via Tauri).
#[tauri::command]
pub async fn list_installed_mods() -> Result<Vec<BarotraumaMod>, String> {
    let conf: Config = read_config()?;
    info!("Listing installed mods for Barotrauma.");
    Ok(BARO_MANAGER
        .write()
        .await
        .set_game_dir(
            &PathBuf::from_str(conf.game_home.as_str())
                .map_err(|e| format!("{}, failed to set game directory.", e))?,
        )
        .refresh_mods()?
        .get_mods()
        .to_vec())
}

#[tauri::command]
pub async fn retrieve_mod_metadata(
    mods: Vec<BarotraumaMod>,
    batch_size: usize,
) -> Result<Vec<BarotraumaMod>, String> {
    get_mod_metadata(mods, batch_size, STEAM_WORKSHOP_CLIENT.read().await.deref())
        .map_err(|e| format!("{}, failed to retrieve mod metadata.", e))
        .await
}

/// Lists all enabled Barotrauma mods found in the configured game directory.
#[tauri::command]
pub async fn list_enabled_mods() -> Result<Vec<BarotraumaMod>, String> {
    let conf: Config = read_config()?;
    info!("Listing enabled mods for Barotrauma.");
    BARO_MANAGER
        .write()
        .await
        .set_game_dir(
            &PathBuf::from_str(conf.game_home.as_str())
                .map_err(|e| format!("{}, failed to set game directory.", e))?,
        )
        .refresh_mods()?
        .enabled_mods()
}

/// Downloads the specified mods using SteamCMD.
#[tauri::command]
pub async fn download_mods(mods: Vec<u64>) -> Result<(), String> {
    let conf: Config = read_config()?;

    info!("Starting to download mods: {:?}", mods);
    STEAMCMD_MANAGER
        .write()
        .await
        .set_steamcmd_home(
            PathBuf::from_str(conf.steamcmd_home.as_str())
                .map_err(|e| format!("{}, failed to set steamcmd home.", e))?,
        )
        .download_mod_par(
            BAROTRAUMA_GAME_ID,
            mods,
            conf.steamcmd_config.unwrap().parallel as usize,
        )
        .await
}
#[tauri::command]
pub async fn list_mod_lists() -> Result<Vec<ModList>, String> {
    let conf: Config = read_config()?;

    BARO_MANAGER
        .write()
        .await
        .set_game_dir(
            &PathBuf::from_str(conf.game_home.as_str())
                .map_err(|e| format!("{}, failed to set game directory.", e))?,
        )
        .discover_mod_lists()
}

/// Retrieves the background image for the UI as a base64-encoded data URL.
///
/// This function reads the current configuration to determine the background image path,
/// reads the image file from disk, and returns it as a base64-encoded data URL suitable
/// for direct use in HTML/CSS. The MIME type is determined automatically based on the
/// file extension.
///
/// # Returns
/// - `Ok(Some(String))`: A base64-encoded data URL of the image if a valid image path
///   is configured and the file exists.
/// - `Ok(None)`: If no background image is configured or the configured path is invalid.
/// - `Err(String)`: If there's an error reading the configuration or the image file.
///
/// # Supported Image Formats
/// - PNG (.png)
/// - JPEG (.jpg, .jpeg)
/// - GIF (.gif)
/// - WebP (.webp)
/// - BMP (.bmp)
/// - SVG (.svg)
///
/// # Configuration
/// The image path is read from `config.ui_config.background_image`. If this path is
/// relative, it's interpreted relative to the application's working directory.
///
/// # Security Note
/// This function reads arbitrary files from the filesystem based on user configuration.
/// In a production environment, consider validating the file path to prevent directory
/// traversal attacks.
#[tauri::command]
pub async fn get_background_image() -> Result<Option<String>, String> {
    let conf: Config = read_config()?;

    if let Some(ui_conf) = conf.ui_config
        && let Some(p_raw) = ui_conf.background_image
        && let Ok(p) = PathBuf::from_str(p_raw.as_str())
        && p.exists()
    {
        process_background(BackgroundConfig {
            image_path: Some(p_raw.to_string()),
            blur_radius: ui_conf.background_blur,
            opacity: ui_conf.background_opacity as f64,
        })
        .map_err(|e| format!("{}, failed to process background image.", e))
    } else {
        Ok(None)
    }
}

/// Returns the version information of the application.
#[tauri::command]
pub fn get_build_info() -> BuildInfo {
    BuildInfo {
        version: crate::rust_built_info::PKG_VERSION.to_string(),
        commit: crate::rust_built_info::GIT_COMMIT_HASH.expect("Can get the commit hash")[..7]
            .into(),
        // BUILT_TIME_UTC format: "Mon Jan  1 12:00:00 2025" — extract "Jan  1 12:00"
        date: crate::rust_built_info::BUILT_TIME_UTC[5..16].into(),
    }
}

#[tauri::command]
pub async fn is_barotrauma_mod(item_id: u64) -> Result<bool, String> {
    STEAM_WORKSHOP_CLIENT
        .read()
        .await
        .get_item(item_id)
        .map_err(|e| format!("{}, failed to retrieve mod metadata.", e))
        .await
        .map(|item: WorkshopItem| item.consumer_app_id == BAROTRAUMA_GAME_ID)
}

#[tauri::command]
pub async fn install_mods(mod_ids: Vec<u64>) -> Result<(), String> {
    let conf: Config = read_config()?;

    let fut: Vec<_> = mod_ids
        .into_iter()
        .map(|item_id| async move {
            match conf.install_strategy {
                s if InstallStrategy::Copy as i32 == s => {
                    let mod_dir: PathBuf = STEAMCMD_MANAGER
                        .read()
                        .await
                        .workshop_item_dir(BAROTRAUMA_GAME_ID, item_id)?;
                    fs_extra::copy_items(
                        &[mod_dir],
                        BARO_MANAGER.read().await.mod_dir()?,
                        &CopyOptions::new().overwrite(true),
                    )
                    .map(|_| ())
                    .map_err(|e| format!("{}, failed to copy mod.", e))
                }
                s if InstallStrategy::Link as i32 == s => {
                    #[cfg(target_os = "windows")]
                    let linker = tokio::fs::symlink_dir;
                    #[cfg(target_os = "linux")]
                    let linker = tokio::fs::symlink;
                    linker(
                        STEAMCMD_MANAGER
                            .read()
                            .await
                            .workshop_item_dir(BAROTRAUMA_GAME_ID, item_id)?,
                        BARO_MANAGER
                            .read()
                            .await
                            .mod_dir()?
                            .join(item_id.to_string()),
                    )
                    .await
                    .map_err(|e| format!("{}, failed to symlink mod.", e))
                }
                _ => Err("Invalid install strategy".to_string()),
            }
        })
        .collect();

    try_join_all(fut).await?;

    Ok(())
}
#[tauri::command]
pub async fn uninstall_mods(mod_ids: Vec<u64>) -> Result<(), String> {
    let id_set: HashSet<u64> = mod_ids.into_iter().collect();
    let manager = BARO_MANAGER.read().await;

    let targets: Vec<_> = manager
        .get_mods()
        .iter()
        .filter(|m| id_set.contains(&m.steam_workshop_id))
        .collect();

    if targets.is_empty() {
        return Err("No matching mods found for the given IDs.".to_string());
    }

    let mut deleted = 0;
    let mut errors: Vec<String> = Vec::new();

    for mod_obj in &targets {
        match &mod_obj.home_dir {
            Some(dir) => {
                let path = Path::new(dir);
                if path.exists() {
                    info!(
                        "Deleting mod '{}' (id={}) at {:?}",
                        mod_obj.name, mod_obj.steam_workshop_id, path
                    );
                    match fs::remove_dir_all(path) {
                        Ok(()) => deleted += 1,
                        Err(e) => {
                            let msg = format!(
                                "Failed to delete '{}' (id={}): {}",
                                mod_obj.name, mod_obj.steam_workshop_id, e
                            );
                            error!("{}", msg);
                            errors.push(msg);
                        }
                    }
                } else {
                    let msg = format!(
                        "Mod '{}' (id={}) path does not exist: {:?}",
                        mod_obj.name, mod_obj.steam_workshop_id, path
                    );
                    warn!("{}", msg);
                    errors.push(msg);
                }
            }
            None => {
                let msg = format!(
                    "Mod '{}' (id={}) has no home directory — cannot delete.",
                    mod_obj.name, mod_obj.steam_workshop_id
                );
                warn!("{}", msg);
                errors.push(msg);
            }
        }
    }

    info!(
        "Uninstall complete: {} deleted, {} errors",
        deleted,
        errors.len()
    );

    if errors.is_empty() {
        Ok(())
    } else if deleted > 0 {
        Err(format!(
            "Deleted {} mod(s), but {} failed: {}",
            deleted,
            errors.len(),
            errors.join("; ")
        ))
    } else {
        Err(format!(
            "Failed to delete all requested mods: {}",
            errors.join("; ")
        ))
    }
}

#[tauri::command]
pub async fn get_mod_occupation(mod_id: u64) -> Result<u64, String> {
    BARO_MANAGER.read().await.get_mod_occupation(mod_id)
}

#[tauri::command]
pub async fn get_mod_hash(mod_id: u64) -> Result<String, String> {
    BARO_MANAGER.read().await.get_mod_hash(mod_id)
}

#[tauri::command]
pub async fn get_workshop_items(item_ids: Vec<u64>) -> Result<Vec<WorkshopItem>, String> {
    let conf: Config = read_config()?;

    STEAM_WORKSHOP_CLIENT
        .read()
        .await
        .get_items_batched(item_ids, conf.metadata_retrieve_batchsize as usize)
        .map_err(|e| format!("{}, failed to retrieve workshop items.", e))
        .await
}

#[tauri::command]
pub async fn create_mod_list(profile_name: String) -> Result<ModList, String> {
    let enabled = list_enabled_mods().await?;
    if enabled.is_empty() {
        return Err("No enabled mods to save as a profile.".to_string());
    }

    // Read config to get the base package
    let conf: Config = read_config()?;
    let game_home = constants::BarotraumaHome::new(
        PathBuf::from_str(&conf.game_home).map_err(|e| format!("{e}, invalid game home path."))?,
    );
    let base_package = {
        let config_file = game_home.player_config_file();
        if config_file.exists() {
            let baro_conf = mod_analyzer::BaroConfig::from_file(&config_file)
                .map_err(|e| format!("{e}, failed to read player config."))?;
            baro_conf
                .core_package()
                .rsplit('/')
                .next()
                .unwrap_or("Vanilla")
                .replace(".xml", "")
        } else {
            "Vanilla".to_string()
        }
    };

    let mod_list = ModList {
        profile_name: profile_name.clone(),
        base_package,
        mods: enabled.iter().map(|m| m.name.clone()).collect(),
    };

    BARO_MANAGER.read().await.save_mod_list(&mod_list)?;

    info!(
        "Created profile '{}' with {} mods",
        profile_name,
        enabled.len()
    );
    Ok(mod_list)
}

#[tauri::command]
pub async fn delete_mod_list(profile_name: String) -> Result<(), String> {
    BARO_MANAGER.read().await.delete_mod_list(&profile_name)?;
    info!("Deleted profile '{}'", profile_name);
    Ok(())
}

#[tauri::command]
pub async fn apply_mod_list(profile_name: String) -> Result<(), String> {
    // 1. Read the profile
    let mod_list_dir = BARO_MANAGER.read().await.mod_list_dir()?.clone();
    let profile_path = mod_list_dir.join(format!("{}.xml", profile_name));
    if !profile_path.exists() {
        return Err(format!("Profile '{}' not found.", profile_name));
    }
    let mod_list = ModList::from_xml_path(&profile_path)
        .map_err(|e| format!("{e}, failed to read profile."))?;

    // 2. Build a name -> id mapping from installed mods
    let installed = list_installed_mods().await?;
    let name_to_id: std::collections::HashMap<String, u64> = installed
        .iter()
        .map(|m| (m.name.clone(), m.steam_workshop_id))
        .collect();

    // 3. Resolve profile mod names to installed mod IDs
    let mut resolved_ids: Vec<u64> = Vec::new();
    for mod_name in &mod_list.mods {
        if let Some(&id) = name_to_id.get(mod_name) {
            resolved_ids.push(id);
        } else {
            warn!(
                "Profile mod '{}' not found in installed mods, skipping.",
                mod_name
            );
        }
    }

    // 4. Read current config and replace regularpackages
    let conf: Config = read_config()?;
    let config_path = PathBuf::from_str(&conf.game_home)
        .map_err(|e| format!("{e}, invalid game home."))?
        .join(constants::BarotraumaHome::PLAYER_CONFIG);

    if !config_path.exists() {
        return Err(
            "Player config file not found. Launch the game at least once first.".to_string(),
        );
    }

    let config_content = fs::read_to_string(&config_path)
        .map_err(|e| format!("{e}, failed to read player config."))?;

    // Build the new regularpackages XML block
    let new_packages: String = resolved_ids
        .iter()
        .map(|id| format!("      <package path=\"LocalMods/{}/filelist.xml\" />", id))
        .collect::<Vec<_>>()
        .join("\n");

    // Replace the <regularpackages>...</regularpackages> block
    let new_config = if let Some(start) = config_content.find("<regularpackages>") {
        if let Some(end) = config_content.find("</regularpackages>") {
            let end_with_tag = end + "</regularpackages>".len();
            format!(
                "{}<regularpackages>\n{}\n    </regularpackages>{}",
                &config_content[..start],
                new_packages,
                &config_content[end_with_tag..]
            )
        } else {
            return Err(
                "Malformed config: found <regularpackages> but no closing tag.".to_string(),
            );
        }
    } else {
        return Err("Malformed config: <regularpackages> section not found.".to_string());
    };

    fs::write(&config_path, new_config)
        .map_err(|e| format!("{e}, failed to write player config."))?;

    // 5. Refresh mod manager state
    BARO_MANAGER.write().await.refresh_mods()?;

    info!(
        "Applied profile '{}' ({} mods resolved from {} in profile)",
        profile_name,
        resolved_ids.len(),
        mod_list.mods.len()
    );
    Ok(())
}
