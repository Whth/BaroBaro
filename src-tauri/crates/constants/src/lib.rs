use directories_next::BaseDirs;
use once_cell::sync::Lazy;
use std::path::PathBuf;

pub const MOD_FILELIST_FILE: &str = "filelist.xml";

pub const LOG_DIR_NAME: &str = "log";

pub const BAROTRAUMA_GAME_ID: usize = 602960;

/// The application name used across the project.
pub const APP_NAME: &str = "BaroBaro";

/// The default configuration file name used by the application.
pub const CONFIG_FILE: &str = "BaroBaro.toml";

/// Returns the path to the user's config directory based on the operating system.
///
/// |Platform | Value                                 | Example                          |
/// | ------- | ------------------------------------- | -------------------------------- |
/// | Linux   | `$XDG_CONFIG_HOME` or `$HOME`/.config/<APPNAME> | /home/alice/.config/app              |
/// | macOS   | `$HOME`/Library/Application Support/<APPNAME>   | /Users/Alice/Library/Application Support/app |
/// | Windows | `{FOLDERID_RoamingAppData}\<APPNAME>`           | C:\Users\Alice\AppData\Roaming\app   |
///
/// # Arguments
/// * `app_name` - The name of the application used when constructing the directory path.
///
/// # Returns
/// An `Option<PathBuf>` representing the roaming directory if available.
fn get_roaming_dir(app_name: &str) -> Option<PathBuf> {
    BaseDirs::new().map(|dirs| dirs.config_dir().join(app_name))
}

/// A global static instance of the user's roaming configuration directory for the application.
pub static ROAMING: Lazy<PathBuf> =
    Lazy::new(|| get_roaming_dir(APP_NAME).expect("Failed to get roaming directory"));

/// A global static instance of the user's global configuration file path for the application.
pub static GLOBAL_CONFIG_FILE: Lazy<PathBuf> = Lazy::new(|| ROAMING.join(CONFIG_FILE));

/// A global static instance of the user's global log directory for the application.
pub static GLOBAL_LOG_DIR: Lazy<PathBuf> = Lazy::new(|| ROAMING.join(LOG_DIR_NAME));

/// Represents the Barotrauma game home directory.
#[derive(Debug)]
pub struct BarotraumaHome {
    home_dir: PathBuf,
    mod_dir: PathBuf,
    mod_list_dir: PathBuf,
}

impl BarotraumaHome {
    pub const MOD_DIR: &'static str = "LocalMods";

    pub const MOD_LIST_DIR: &'static str = "ModLists";

    pub fn new(home_dir: PathBuf) -> BarotraumaHome {
        BarotraumaHome {
            home_dir: home_dir.clone(),
            mod_dir: home_dir.join(Self::MOD_DIR),
            mod_list_dir: home_dir.join(Self::MOD_LIST_DIR),
        }
    }

    pub fn set_home_dir(&mut self, home_dir: &PathBuf) {
        self.home_dir = home_dir.clone();
        self.mod_dir = home_dir.join(Self::MOD_DIR);
        self.mod_list_dir = home_dir.join(Self::MOD_LIST_DIR);
    }

    pub fn mod_dir(&self) -> &PathBuf {
        &self.mod_dir
    }

    pub fn mod_list_dir(&self) -> &PathBuf {
        &self.mod_list_dir
    }

    pub fn home_dir(&self) -> &PathBuf {
        &self.home_dir
    }
}
