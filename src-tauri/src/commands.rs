use configuration::Config;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use constants::{GLOBAL_CONFIG_FILE, ROAMING};
use mod_analyzer::BarotraumaMod;

use crate::once::BARO_MANAGER;
use toml::{from_str, to_string_pretty};

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

#[tauri::command]
pub fn list_installed_mods() -> Result<Vec<BarotraumaMod>, String> {
    let conf: Config = read_config()?;
    BARO_MANAGER.set_game_dir(
        &PathBuf::from_str(conf.game_home.as_str())
            .map_err(|e| format!("{}, failed to set game directory.", e))?,
    )?;
    todo!();
}
