use mod_analyzer::BarotraumaModManager;
use std::sync::LazyLock;
use steam_api::SteamWorkShopClient;
use steamcmd_rs::SteamCMD;
use tokio::sync::RwLock;
/// A static instance of BarotraumaModManager
pub static BARO_MANAGER: LazyLock<RwLock<BarotraumaModManager>> =
    LazyLock::new(|| BarotraumaModManager::default().into());

/// A static instance of SteamCMD
pub static STEAMCMD_MANAGER: LazyLock<RwLock<SteamCMD>> = LazyLock::new(|| SteamCMD::default().into());

pub static STEAM_WORKSHOP_CLIENT: LazyLock<RwLock<SteamWorkShopClient>> =
    LazyLock::new(|| SteamWorkShopClient::new().into());
