mod config;
pub use config::*;
use figment::providers::{Format, Toml};
use figment::value::{Dict, Map};
use figment::{Error, Figment, Profile, Provider};
use logger::info;
use std::path::Path;
use toml::to_string_pretty;

impl Config {
    pub fn default_settings() -> Self {
        Self {
            loglevel: Level::Info as i32,
            game_home: "".to_string(),
            steamcmd_home: "".to_string(),
            metadata_retrieve_batchsize: 10,
            install_strategy: InstallStrategy::Copy as i32,
            steamcmd_config: Some(SteamCmdConfig {
                username: None,
                password: None,
                parallel: 3,
            }),
            ui_config: Some(UiConfig {
                theme: Theme::Light as i32,
                language: Language::En as i32,
                background_image: None,
                background_opacity: 0.8,
                background_blur: 5,
                foreground_opacity: 0.9,
            }),
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        info!("Loading config from {:?}", path.as_ref());
        Figment::new()
            .join(Toml::file(path))
            .join(Self::default_settings())
            .extract()
            .map_err(|e| e.into())
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let s = to_string_pretty(self)?;
        std::fs::write(path, s)?;
        Ok(())
    }
}

impl Provider for Config {
    fn metadata(&self) -> figment::Metadata {
        figment::Metadata::named("Barotrauma Mod Manager Config")
    }

    fn data(&self) -> Result<Map<Profile, Dict>, Error> {
        figment::providers::Serialized::defaults(Config::default_settings()).data()
    }
}
