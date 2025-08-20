mod commands;

use transmission::Config;

use commands::*;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), String> {
    let conf: Config = read_config()?;

    logger::init_logger(conf.loglevel.as_str())?;
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            read_config,
            write_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
