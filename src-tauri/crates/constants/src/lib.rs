use std::path::PathBuf;

pub const BAROTRAUMA_GAME_ID:usize=602960;
pub struct BarotraumaHome {
    home_dir: PathBuf,
    mod_dir: PathBuf,
    mod_list_dir: PathBuf,
}

impl BarotraumaHome {

    pub const  MOD_DIR: &'static str = "LocalMods";

    pub const MOD_LIST_DIR: &'static str = "ModLists";

    pub fn new(home_dir: PathBuf) -> BarotraumaHome {
        BarotraumaHome {
            home_dir: home_dir.clone(),
            mod_dir: home_dir.join(Self::MOD_DIR),
            mod_list_dir: home_dir.join(Self::MOD_LIST_DIR),
        }
    }

    pub fn mod_dir(&self) -> &PathBuf {
        &self.mod_dir
    }

    pub fn mod_list_dir(&self) -> &PathBuf {
        &self.mod_list_dir
    }

    pub fn home_dir(&self)-> &PathBuf {
        &self.home_dir
    }

}






