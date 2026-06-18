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
use std::collections::HashMap;
use std::fs;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::str::FromStr;

use crate::build_info::BuildInfo;
use crate::once::{BARO_MANAGER, STEAM_WORKSHOP_CLIENT, STEAMCMD_MANAGER};
use constants::{BAROTRAUMA_GAME_ID, GLOBAL_CONFIG_FILE, ROAMING};
use futures::TryFutureExt;
use futures::future::try_join_all;
use imagen::{BackgroundConfig, process_background};
use logger::{debug, error, info, warn};
use mod_analyzer::{BarotraumaMod, ModList, parse_dependencies};
use steam_api::WorkshopItem;

/// Recursively copies a directory, overwriting existing files.
fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let dest_path = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_recursive(&entry.path(), &dest_path)?;
        } else {
            fs::copy(entry.path(), &dest_path)?;
        }
    }
    Ok(())
}

/// Ephemeral status returned by `check_mod_updates`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModUpdateStatus {
    pub mod_id: u64,
    pub needs_update: bool,
    pub stored_hash: Option<String>,
    pub current_hash: Option<String>,
}

/// A missing dependency: mod A declares a dependency on mod B,
/// but mod B is not among enabled mods.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MissingDependency {
    /// Name of the dependent mod.
    pub mod_name: String,
    /// Steam Workshop ID of the dependent mod.
    pub mod_steam_id: u64,
    /// Name of the missing dependency.
    pub dependency_name: String,
    /// Steam Workshop ID of the missing dependency, if declared.
    pub dependency_steam_id: Option<u64>,
}

/// Result of `detect_mod_conflicts`: lists missing dependencies among enabled mods.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConflictReport {
    pub missing_dependencies: Vec<MissingDependency>,
}

/// Path to the persistent hash cache file.
fn hash_cache_path() -> PathBuf {
    ROAMING.join("mod_hashes.json")
}

/// Load the hash cache from disk.
fn load_hash_cache() -> HashMap<u64, String> {
    let path = hash_cache_path();
    if path.exists() {
        fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    } else {
        HashMap::new()
    }
}

/// Persist the hash cache to disk.
fn save_hash_cache(cache: &HashMap<u64, String>) -> Result<(), String> {
    let path = hash_cache_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create hash cache directory: {e}"))?;
    }
    let json = serde_json::to_string_pretty(cache)
        .map_err(|e| format!("Failed to serialize hash cache: {e}"))?;
    fs::write(&path, json)
        .map_err(|e| format!("Failed to write hash cache: {e}"))
}

/// Writes a new mod order to the player config's `<regularpackages>` block.
fn write_regularpackages(ordered_ids: &[u64]) -> Result<(), String> {
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

    let new_packages: String = ordered_ids
        .iter()
        .map(|id| format!("      <package path=\"LocalMods/{}/filelist.xml\" />", id))
        .collect::<Vec<_>>()
        .join("\n");

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
        .map_err(|e| format!("{e}, failed to write player config."))
}

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
        .iter()
        .copied()
        .map(|item_id| async move {
            match conf.install_strategy {
                s if InstallStrategy::Copy as i32 == s => {
                    let mod_dir: PathBuf = STEAMCMD_MANAGER
                        .read()
                        .await
                        .workshop_item_dir(BAROTRAUMA_GAME_ID, item_id)?;
                    let dest = BARO_MANAGER.read().await.mod_dir()?.join(
                        mod_dir
                            .file_name()
                            .ok_or_else(|| "Invalid mod directory name".to_string())?,
                    );
                    copy_dir_recursive(&mod_dir, &dest)
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

    // Refresh the mod list so newly installed mods are discoverable
    BARO_MANAGER.write().await.refresh_mods()?;

    // Compute and persist hashes for the installed mods
    let mut cache = load_hash_cache();
    {
        let manager = BARO_MANAGER.read().await;
        for &mod_id in &mod_ids {
            if let Ok(hash) = manager.get_mod_hash(mod_id) {
                cache.insert(mod_id, hash);
            }
        }
    }
    save_hash_cache(&cache)?;

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

    // Prune hash cache for uninstalled mods
    if deleted > 0 {
        let mut cache = load_hash_cache();
        for &mod_id in &id_set {
            cache.remove(&mod_id);
        }
        if let Err(e) = save_hash_cache(&cache) {
            warn!("Failed to prune hash cache: {}", e);
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
pub async fn check_mod_updates(mod_ids: Vec<u64>) -> Result<Vec<ModUpdateStatus>, String> {
    let cache = load_hash_cache();

    // Clone mod objects under read lock, then drop lock before hashing
    let mods: Vec<BarotraumaMod> = {
        let manager = BARO_MANAGER.read().await;
        mod_ids
            .iter()
            .filter_map(|&id| {
                manager.get_mods().iter().find(|m| m.steam_workshop_id == id).cloned()
            })
            .collect()
    };

    // Compute hashes without holding the lock
    let mut results = Vec::with_capacity(mod_ids.len());
    for mod_obj in mods {
        let mod_id = mod_obj.steam_workshop_id;
        let stored_hash = cache.get(&mod_id).cloned();
        let current_hash = mod_obj.mod_hash().ok();
        let needs_update = match (&stored_hash, &current_hash) {
            (Some(stored), Some(current)) => stored != current,
            _ => true,
        };
        results.push(ModUpdateStatus {
            mod_id,
            needs_update,
            stored_hash,
            current_hash,
        });
    }
    Ok(results)
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
    // Clear active profile if we just deleted it
    let conf = read_config()?;
    if conf.active_profile.as_deref() == Some(&profile_name) {
        let mut conf = conf;
        conf.active_profile = None;
        write_config(conf)?;
    }
    info!("Deleted profile '{}'", profile_name);
    Ok(())
}

/// Result of comparing two mod profiles.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProfileDiff {
    only_in_a: Vec<String>,
    only_in_b: Vec<String>,
    in_both: Vec<String>,
}

/// Renames a profile by reading its XML, updating the name, and saving under the new name.
#[tauri::command]
pub async fn rename_profile(old_name: String, new_name: String) -> Result<ModList, String> {
    let manager = BARO_MANAGER.read().await;
    let mod_list_dir = manager.mod_list_dir()?.clone();
    drop(manager);

    let old_path = mod_list_dir.join(format!("{}.xml", old_name));
    if !old_path.exists() {
        return Err(format!("Profile '{}' not found.", old_name));
    }

    let mut mod_list = ModList::from_xml_path(&old_path)
        .map_err(|e| format!("{e}, failed to read profile."))?;
    mod_list.profile_name = new_name.clone();

    let manager = BARO_MANAGER.read().await;
    manager.save_mod_list(&mod_list)?;
    manager.delete_mod_list(&old_name)?;

    // Update active profile if the renamed one was active
    let conf = read_config()?;
    if conf.active_profile.as_deref() == Some(&old_name) {
        let mut conf = conf;
        conf.active_profile = Some(new_name.clone());
        write_config(conf)?;
    }

    info!("Renamed profile '{}' to '{}'", old_name, new_name);
    Ok(mod_list)
}

/// Compares two mod profiles, returning the diff of their mod lists.
#[tauri::command]
pub async fn compare_profiles(
    name_a: String,
    name_b: String,
) -> Result<ProfileDiff, String> {
    let manager = BARO_MANAGER.read().await;
    let mod_list_dir = manager.mod_list_dir()?.clone();
    drop(manager);

    let path_a = mod_list_dir.join(format!("{}.xml", name_a));
    let path_b = mod_list_dir.join(format!("{}.xml", name_b));

    if !path_a.exists() {
        return Err(format!("Profile '{}' not found.", name_a));
    }
    if !path_b.exists() {
        return Err(format!("Profile '{}' not found.", name_b));
    }

    let mod_list_a = ModList::from_xml_path(&path_a)
        .map_err(|e| format!("{e}, failed to read profile '{}'.", name_a))?;
    let mod_list_b = ModList::from_xml_path(&path_b)
        .map_err(|e| format!("{e}, failed to read profile '{}'.", name_b))?;

    let set_a: HashSet<&str> = mod_list_a.mods.iter().map(|s| s.as_str()).collect();
    let set_b: HashSet<&str> = mod_list_b.mods.iter().map(|s| s.as_str()).collect();

    let only_in_a: Vec<String> = set_a.difference(&set_b).map(|s| s.to_string()).collect();
    let only_in_b: Vec<String> = set_b.difference(&set_a).map(|s| s.to_string()).collect();
    let in_both: Vec<String> = set_a.intersection(&set_b).map(|s| s.to_string()).collect();

    Ok(ProfileDiff {
        only_in_a,
        only_in_b,
        in_both,
    })
}

/// Exports a profile XML file to the given path.
#[tauri::command]
pub async fn export_profile(profile_name: String, export_path: String) -> Result<(), String> {
    let manager = BARO_MANAGER.read().await;
    let mod_list_dir = manager.mod_list_dir()?.clone();
    drop(manager);

    let profile_path = mod_list_dir.join(format!("{}.xml", profile_name));
    if !profile_path.exists() {
        return Err(format!("Profile '{}' not found.", profile_name));
    }

    fs::copy(&profile_path, &export_path)
        .map_err(|e| format!("{e}, failed to export profile."))?;

    info!("Exported profile '{}' to '{}'", profile_name, export_path);
    Ok(())
}

/// Imports a profile from an XML file at the given path.
#[tauri::command]
pub async fn import_profile(path: String) -> Result<ModList, String> {
    let mod_list = ModList::from_xml_path(&path)
        .map_err(|e| format!("{e}, failed to read profile file."))?;

    let manager = BARO_MANAGER.read().await;
    manager.save_mod_list(&mod_list)?;

    info!("Imported profile '{}' from '{}'", mod_list.profile_name, path);
    Ok(mod_list)
}

/// Backs up the current player config before it gets overwritten.
fn backup_player_config() -> Result<(), String> {
    let conf = read_config()?;
    let config_path = PathBuf::from_str(&conf.game_home)
        .map_err(|e| format!("{e}, invalid game home."))?
        .join(constants::BarotraumaHome::PLAYER_CONFIG);

    if !config_path.exists() {
        return Ok(());
    }

    let backup_dir = ROAMING.join("config_backups");
    fs::create_dir_all(&backup_dir)
        .map_err(|e| format!("{e}, failed to create backup directory."))?;

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|e| format!("{e}"))?
        .as_secs();

    let backup_path = backup_dir.join(format!("config_player_{}.xml", timestamp));
    fs::copy(&config_path, &backup_path)
        .map_err(|e| format!("{e}, failed to create backup."))?;

    // Prune old backups, keep last 5
    let mut backups: Vec<_> = fs::read_dir(&backup_dir)
        .map_err(|e| format!("{e}"))?
        .filter_map(Result::ok)
        .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
        .filter(|e| e.file_name().to_string_lossy().starts_with("config_player_"))
        .collect();

    backups.sort_by_key(|e| e.file_name());

    let to_delete = backups.len().saturating_sub(5);
    for entry in backups.iter().take(to_delete) {
        fs::remove_file(entry.path()).ok();
    }

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

    // 4. Write the resolved order to player config
    backup_player_config()?;
    write_regularpackages(&resolved_ids)?;

    // 5. Refresh mod manager state
    BARO_MANAGER.write().await.refresh_mods()?;

    // 6. Record the applied profile as active
    let mut conf = read_config()?;
    conf.active_profile = Some(profile_name.clone());
    write_config(conf)?;

    info!(
        "Applied profile '{}' ({} mods resolved from {} in profile)",
        profile_name,
        resolved_ids.len(),
        mod_list.mods.len()
    );
    Ok(())
}

#[tauri::command]
pub fn set_active_profile(profile_name: String) -> Result<(), String> {
    let mut conf = read_config()?;
    conf.active_profile = Some(profile_name);
    write_config(conf)
}

#[tauri::command]
pub fn clear_active_profile() -> Result<(), String> {
    let mut conf = read_config()?;
    conf.active_profile = None;
    write_config(conf)
}

#[tauri::command]
pub async fn reorder_enabled_mods(ordered_ids: Vec<u64>) -> Result<(), String> {
    // Validate that all IDs correspond to installed mods
    let installed_ids: HashSet<u64> = {
        let manager = BARO_MANAGER.read().await;
        manager.get_mods().iter().map(|m| m.steam_workshop_id).collect()
    };
    let invalid: Vec<u64> = ordered_ids.iter().copied().filter(|id| !installed_ids.contains(id)).collect();
    if !invalid.is_empty() {
        return Err(format!("Invalid mod IDs: {:?}", invalid));
    }
    write_regularpackages(&ordered_ids)?;
    BARO_MANAGER.write().await.refresh_mods()?;
    info!("Reordered enabled mods ({} mods)", ordered_ids.len());
    Ok(())
}

/// Detects missing dependencies among currently enabled mods.
///
/// Reads each enabled mod's content.xml, extracts `<package>` dependency
/// declarations, and reports any dependency that is not found among the
/// set of enabled mods (matched by either Steam Workshop ID or name).
#[tauri::command]
pub async fn detect_mod_conflicts() -> Result<ConflictReport, String> {
    let manager = BARO_MANAGER.read().await;
    let enabled = manager.enabled_mods().map_err(|e| format!("{e}, failed to get enabled mods."))?;
    drop(manager);

    // Build lookup sets from enabled mods
    let enabled_ids: HashSet<u64> = enabled.iter().map(|m| m.steam_workshop_id).filter(|id| *id != 0).collect();
    let enabled_names: HashSet<&str> = enabled.iter().map(|m| m.name.as_str()).collect();

    let mut missing = Vec::new();

    for mod_obj in &enabled {
        let home_dir = match &mod_obj.home_dir {
            Some(d) => std::path::PathBuf::from(d),
            None => continue,
        };
        let xml_path = home_dir.join(constants::MOD_FILELIST_FILE);
        let xml = match std::fs::read_to_string(&xml_path) {
            Ok(x) => x,
            Err(_) => continue,
        };
        let deps = match parse_dependencies(&xml) {
            Ok(d) => d,
            Err(_) => continue,
        };

        for dep in &deps {
            let found = match dep.steam_workshop_id {
                Some(id) if id != 0 => enabled_ids.contains(&id),
                _ => enabled_names.contains(dep.name.as_str()),
            };
            if !found {
                missing.push(MissingDependency {
                    mod_name: mod_obj.name.clone(),
                    mod_steam_id: mod_obj.steam_workshop_id,
                    dependency_name: dep.name.clone(),
                    dependency_steam_id: dep.steam_workshop_id,
                });
            }
        }
    }

    Ok(ConflictReport {
        missing_dependencies: missing,
    })
}

/// Result of checking a single mod for workshop updates.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkshopUpdateStatus {
    pub mod_id: u64,
    pub mod_name: String,
    pub has_update: bool,
    pub local_last_modified: Option<u64>,
    pub workshop_last_updated: Option<u64>,
}

/// Checks installed mods against Steam Workshop for available updates.
#[tauri::command]
pub async fn check_workshop_updates() -> Result<Vec<WorkshopUpdateStatus>, String> {
    let enabled = BARO_MANAGER.read().await.enabled_mods()?;
    if enabled.is_empty() {
        return Ok(Vec::new());
    }

    let ids: Vec<u64> = enabled.iter().map(|m| m.steam_workshop_id).filter(|&id| id > 0).collect();
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    let conf: Config = read_config()?;
    let workshop_items = STEAM_WORKSHOP_CLIENT
        .read()
        .await
        .get_items_batched(ids.clone(), conf.metadata_retrieve_batchsize as usize)
        .map_err(|e| format!("{e}, failed to check workshop updates."))
        .await?;

    let workshop_map: HashMap<u64, &steam_api::WorkshopItem> = workshop_items
        .iter()
        .map(|item| (item.published_file_id, item))
        .collect();

    let results: Vec<WorkshopUpdateStatus> = enabled
        .iter()
        .filter(|m| m.steam_workshop_id > 0)
        .map(|m| {
            let workshop = workshop_map.get(&m.steam_workshop_id);
            let workshop_time = workshop.map(|w| w.time_updated);
            let local_time = m.last_modified;
            let has_update = match (local_time, workshop_time) {
                (Some(local), Some(workshop)) => workshop > local,
                (None, Some(_)) => true,
                _ => false,
            };
            WorkshopUpdateStatus {
                mod_id: m.steam_workshop_id,
                mod_name: m.name.clone(),
                has_update,
                local_last_modified: local_time,
                workshop_last_updated: workshop_time,
            }
        })
        .collect();

    Ok(results)
}

/// Network connectivity status.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkStatus {
    pub steam_api: bool,
    pub steamcmd_available: bool,
}

/// Checks network connectivity to Steam services.
#[tauri::command]
pub async fn check_network_status() -> NetworkStatus {
    let steam_api_ok = STEAM_WORKSHOP_CLIENT
        .read()
        .await
        .get_items_batched(vec![602960], 1)  // Barotrauma itself as a test
        .await
        .is_ok();

    let conf = read_config().ok();
    let steamcmd_path = conf
        .as_ref()
        .and_then(|c| if c.steamcmd_home.is_empty() { None } else { Some(&c.steamcmd_home) });
    let steamcmd_available = steamcmd_path
        .map(|p| Path::new(p).join("steamcmd.exe").exists() || Path::new(p).join("steamcmd").exists())
        .unwrap_or(false);

    NetworkStatus {
        steam_api: steam_api_ok,
        steamcmd_available,
    }
}

/// Popular Barotrauma workshop mod IDs for browsing.
const POPULAR_MOD_IDS: &[u64] = &[
    2942414988, 2942414988, // Placeholder — will be replaced with real IDs
];

/// Returns popular Barotrauma mods from the Steam Workshop.
#[tauri::command]
pub async fn get_popular_mods() -> Result<Vec<WorkshopItem>, String> {
    let conf: Config = read_config()?;
    let ids: Vec<u64> = POPULAR_MOD_IDS.to_vec();
    STEAM_WORKSHOP_CLIENT
        .read()
        .await
        .get_items_batched(ids, conf.metadata_retrieve_batchsize as usize)
        .map_err(|e| format!("{e}, failed to fetch popular mods."))
        .await
}
