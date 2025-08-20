use crate::{BarotraumaMod, ModList, hash_directory};
use constants::BarotraumaHome;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Default, Debug)]
pub struct BarotraumaModManager {
    game_home: Option<BarotraumaHome>,
    mods: Vec<BarotraumaMod>,
}

impl BarotraumaModManager {
    fn discover_mods(mod_dir: &PathBuf) -> Vec<BarotraumaMod> {
        WalkDir::new(&mod_dir)
            .max_depth(1)
            .min_depth(1)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .map(|entry| BarotraumaMod::from_mod_dir(entry.path()))
            .filter_map(|result| result.ok())
            .collect()
    }

    pub fn set_game_dir(&mut self, game_dir: &PathBuf) -> Result<&mut Self, String> {
        if let Some(ref mut game_home) = self.game_home {
            game_home.set_home_dir(game_dir);
            Ok(self)
        } else {
            Err("Game home not set".to_string())
        }
    }
    pub fn from_game_dir(game_dir: PathBuf) -> BarotraumaModManager {
        let mods = BarotraumaModManager::discover_mods(&game_dir);
        BarotraumaModManager {
            game_home: Some(BarotraumaHome::new(game_dir)),
            mods,
        }
    }

    pub fn calc_mod_hash(&self, name: &str) -> Result<String, String> {
        if let Some(baro_mod) = self.mods.iter().find(|mod_obj| mod_obj.name == name)
            && let Some(ref game_home) = baro_mod.home_dir
        {
            hash_directory(game_home).map_err(|e| format!("{e}, failed to hash directory."))
        } else {
            Err("Game home not set".to_string())
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
}
