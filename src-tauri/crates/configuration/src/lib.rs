mod config;

pub use config::*;

impl Config {
    pub fn default_settings() -> Self {
        Self {
            loglevel: Level::Info as i32,
            game_home: "".to_string(),
            steamcmd_home: "".to_string(),
            steamcmd_config: Some(SteamCmdConfig {
                username: "".to_string(),
                password: "".to_string(),
                parallel: 3,
            }),
            ui_config: Some(UiConfig {
                theme: Theme::Dark as i32,
                language: Language::En as i32,
                accent_color: "#0969da".to_string(),
                background_image: "".to_string(),
                background_opacity: 0.2,
                background_blur: 5,
            }),
        }
    }
}
