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
        }
    }
}
