use crate::BarotraumaMod;
use constants::BarotraumaHome;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct BarotraumaModManager {
    game_home: BarotraumaHome,
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

    pub fn from_game_dir(game_dir: PathBuf) -> BarotraumaModManager {
        let mods = BarotraumaModManager::discover_mods(&game_dir);
        BarotraumaModManager {
            game_home: BarotraumaHome::new(game_dir),
            mods,
        }
    }
    pub fn get_mods(&self) -> &Vec<BarotraumaMod> {
        &self.mods
    }
    pub fn refresh_mods(&mut self) {
        self.mods = BarotraumaModManager::discover_mods(&self.game_home.mod_dir());
    }
}
