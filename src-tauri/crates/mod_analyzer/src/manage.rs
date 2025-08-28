use crate::config_analyzer::BaroConfig;
use crate::retrieve::retrieve_mod_metadata;
use crate::{BarotraumaMod, ModList};
use constants::BarotraumaHome;
use rayon::prelude::*;
use std::path::PathBuf;
use steam_api::SteamWorkShopClient;
use walkdir::WalkDir;
#[derive(Default, Debug)]
pub struct BarotraumaModManager {
    game_home: Option<BarotraumaHome>,
    mods: Vec<BarotraumaMod>,
}

impl BarotraumaModManager {
    fn discover_mods(mod_dir: &PathBuf) -> Vec<BarotraumaMod> {
        let mut mods: Vec<BarotraumaMod> = WalkDir::new(&mod_dir)
            .max_depth(1)
            .min_depth(1)
            .into_iter()
            .par_bridge()
            .filter_map(Result::ok)
            .filter_map(|entry| BarotraumaMod::from_mod_dir(entry.path()).ok())
            .collect();
        mods.sort_by_key(|mod_obj| mod_obj.steam_workshop_id);
        mods
    }

    pub fn mod_dir(&self) -> Result<&PathBuf, String> {
        if let Some(ref game_home) = self.game_home {
            Ok(game_home.mod_dir())
        } else {
            Err("Game home not set".to_string())
        }
    }

    pub async fn retrieve_metadata(
        &mut self,
        client: &SteamWorkShopClient,
        batch_size: usize,
    ) -> Result<&mut Self, String> {
        let mods = self.mods.clone();
        self.mods = retrieve_mod_metadata(mods, batch_size, &client)
            .await
            .map_err(|e| format!("{e}, failed to retrieve mod metadata."))?;
        Ok(self)
    }

    pub fn set_game_dir(&mut self, game_dir: &PathBuf) -> &mut Self {
        if let Some(ref mut game_home) = self.game_home {
            game_home.set_home_dir(game_dir);
        } else {
            self.game_home = Some(BarotraumaHome::new(game_dir.into()))
        }
        self
    }
    pub fn from_game_dir(game_dir: PathBuf) -> BarotraumaModManager {
        let mods = BarotraumaModManager::discover_mods(&game_dir);
        BarotraumaModManager {
            game_home: Some(BarotraumaHome::new(game_dir)),
            mods,
        }
    }

    pub fn discover_mod_lists(&self) -> Result<Vec<ModList>, String> {
        if let Some(ref game_home) = self.game_home {
            Ok(WalkDir::new(game_home.mod_list_dir())
                .min_depth(1)
                .max_depth(1)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|entry| entry.file_type().is_file())
                .map(|entry| ModList::from_xml_path(entry.path()))
                .filter_map(Result::ok)
                .collect::<Vec<ModList>>())
        } else {
            Err("Game home not set".to_string())
        }
    }
    pub fn get_mods(&self) -> &Vec<BarotraumaMod> {
        &self.mods
    }
    pub fn refresh_mods(&mut self) -> Result<&mut Self, String> {
        if let Some(ref game_home) = self.game_home {
            self.mods = BarotraumaModManager::discover_mods(&game_home.mod_dir());
            Ok(self)
        } else {
            Err("Game home not set".to_string())
        }
    }

    pub fn enabled_mods(&self) -> Result<Vec<BarotraumaMod>, String> {
        if let Some(ref game_home) = self.game_home {
            let conf: BaroConfig =
                BaroConfig::from_file(game_home.player_config_file()).map_err(|e| {
                    format!(
                        "{e}, config file {}, failed to parse config.",
                        game_home.player_config_file().display()
                    )
                })?;

            Ok(conf
                .mods()
                .into_iter()
                .map(|mod_entry| {
                    let mut p = game_home.home_dir().to_owned();
                    p.push(mod_entry.path());
                    BarotraumaMod::from_path(p)
                })
                .filter_map(Result::ok)
                .collect::<Vec<_>>())
        } else {
            Err("Game home not set".to_string())
        }
    }
}
