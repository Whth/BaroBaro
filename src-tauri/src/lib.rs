use transmission::Config;

#[tauri::command]
async fn write_config(config: Config) -> Result<(), ()> {
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}
#[tauri::command]
async fn read_config() -> Result<Config, ()> {
    let config_file = std::fs::File::open("config.json").unwrap();
    let config: Config = serde_json::from_reader(config_file).unwrap();
    Ok(config)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
