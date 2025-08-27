//! Configuration management and mod listing functionality for the Barotrauma mod manager.
//!
//! This module provides Tauri commands to:
//! - Read and write persistent configuration settings.
//! - List installed mods by analyzing the game directory.
//!
//! The configuration is stored in TOML format under a global config file path,
//! typically in the OS-specific roaming/app data directory.

use configuration::Config;
use std::collections::HashMap;
use std::fs;
use std::fs::soft_link;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use constants::{BAROTRAUMA_GAME_ID, GLOBAL_CONFIG_FILE, ROAMING};
use mod_analyzer::{BarotraumaMod, ModList};

use crate::build_info::BuildInfo;
use crate::once::{BARO_MANAGER, STEAM_WORKSHOP_CLIENT, STEAMCMD_MANAGER};
use base64::{Engine as _, engine::general_purpose};
use futures::TryFutureExt;
use logger::{debug, info};
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
        .iter()
        .map(|baro_mod| baro_mod.clone())
        .collect::<Vec<_>>())
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
pub async fn download_mods(mods: Vec<usize>) -> Result<(), String> {
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
pub fn get_background_image() -> Result<Option<String>, String> {
    // Read current configuration
    let config: Config = read_config()?;

    if let Some(ui_conf) = config.ui_config
        && let Ok(image_path) = PathBuf::from_str(ui_conf.background_image.as_str())
        && image_path.exists()
        && let Ok(image_data) = fs::read(&image_path)
    {
        info!("Reading background image from {}", image_path.display());
        // Determine MIME type based on file extension
        let mime_type = match image_path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase())
        {
            Some(ext) => match ext.as_str() {
                "png" => "image/png",
                "jpg" | "jpeg" => "image/jpeg",
                "gif" => "image/gif",
                "webp" => "image/webp",
                "bmp" => "image/bmp",
                "svg" => "image/svg+xml",
                _ => "image/png", // Default fallback
            },
            None => "image/png", // Default if no extension
        };

        // Encode as base64
        let base64_data = general_purpose::STANDARD.encode(&image_data);
        // Return as data URL
        Ok(Some(format!("data:{};base64,{}", mime_type, base64_data)))
    } else {
        info!("No background image configured or image does not exist.");
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
        date: crate::rust_built_info::BUILT_TIME_UTC[5..16].into(),
    }
}
