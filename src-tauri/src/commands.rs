//! Configuration management and mod listing functionality for the Barotrauma mod manager.
//!
//! This module provides Tauri commands to:
//! - Read and write persistent configuration settings.
//! - List installed mods by analyzing the game directory.
//!
//! The configuration is stored in TOML format under a global config file path,
//! typically in the OS-specific roaming/app data directory.

use configuration::Config;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use constants::{BAROTRAUMA_GAME_ID, GLOBAL_CONFIG_FILE, ROAMING};
use mod_analyzer::{BarotraumaMod, ModList};

use crate::once::{BARO_MANAGER, STEAMCMD_MANAGER};
use toml::{from_str, to_string_pretty};

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
    fs::write(
        GLOBAL_CONFIG_FILE.clone(),
        to_string_pretty(&config).map_err(|e| format!("{}, failed to write config file.", e))?,
    )
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
        let config_file = fs::read_to_string(GLOBAL_CONFIG_FILE.clone())
            .map_err(|e| format!("{}, failed to read config file.", e))?;
        let config: Config =
            from_str(&config_file).map_err(|e| format!("{}, failed to parse config file.", e))?;
        Ok(config)
    } else {
        Ok(Config::default_settings())
    }
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
    Ok(BARO_MANAGER
        .write()
        .await
        .set_game_dir(
            &PathBuf::from_str(conf.game_home.as_str())
                .map_err(|e| format!("{}, failed to set game directory.", e))?,
        )?
        .refresh_mods()?
        .get_mods()
        .iter()
        .map(|baro_mod| baro_mod.clone())
        .collect::<Vec<_>>())
}

#[tauri::command]
pub async fn download_mods(mods: Vec<usize>) -> Result<(), String> {
    let conf: Config = read_config()?;

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
        )?
        .discover_mod_lists()
}
