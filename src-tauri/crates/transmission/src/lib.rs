mod transmission;

pub use transmission::*;

impl Config {
    pub fn default_settings() -> Self {
        Self {
            loglevel: "INFO".to_string(),
            game_home: "path\\to\\game".to_string(),
            steamcmd_home: "path\\to\\steamcmd".to_string(),
            steamcmd_config: Some(SteamCmdConfig::default()),
        }
    }
}
