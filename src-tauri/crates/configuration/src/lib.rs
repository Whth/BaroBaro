mod config;

pub use config::*;

impl Config {
    pub fn default_settings() -> Self {
        Self {
            loglevel: Level::Info as i32,
            game_home: "path\\to\\game".to_string(),
            steamcmd_home: "path\\to\\steamcmd".to_string(),
            steamcmd_config: Some(SteamCmdConfig::default()),
        }
    }
}
