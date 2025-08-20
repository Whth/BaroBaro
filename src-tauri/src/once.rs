use mod_analyzer::BarotraumaModManager;
use once_cell::sync::Lazy;
use std::convert::Into;
use steamcmd_rs::SteamCMD;
use tokio::sync::RwLock;
/// A static instance of BarotraumaModManager
pub static BARO_MANAGER: Lazy<RwLock<BarotraumaModManager>> =
    Lazy::new(|| BarotraumaModManager::default().into());

/// A static instance of SteamCMD
pub static STEAMCMD_MANAGER: Lazy<RwLock<SteamCMD>> = Lazy::new(|| SteamCMD::default().into());
