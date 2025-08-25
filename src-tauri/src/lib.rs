mod commands;
mod once;

use configuration::{Config, Level};

use commands::*;
pub(crate) mod rust_built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub(crate) mod build_info;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), String> {
    let conf: Config = read_config()?;

    let level = Level::try_from(conf.loglevel).map_err(|e| format!("{}, invalid loglevel.", e))?;
    logger::init_logger(level.as_str_name())?;
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            read_config,
            write_config,
            list_installed_mods,
            download_mods,
            list_mod_lists,
            get_background_image,
            get_default_config,
            get_build_info,
            list_enabled_mods,
            retrieve_mod_metadata
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
