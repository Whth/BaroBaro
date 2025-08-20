use crate::BarotraumaMod;
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
