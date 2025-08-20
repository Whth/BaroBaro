use mod_analyzer::BarotraumaModManager;
use once_cell::sync::Lazy;
use steamcmd_rs::SteamCMD;

/// A static instance of BarotraumaModManager
pub const BARO_MANAGER: Lazy<BarotraumaModManager> = Lazy::new(|| BarotraumaModManager::default());

/// A static instance of SteamCMD
pub const STEAMCMD_MANAGER: Lazy<SteamCMD> = Lazy::new(|| SteamCMD::default());
