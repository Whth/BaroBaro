use mod_analyzer::BarotraumaModManager;
use once_cell::sync::Lazy;

pub const BARO_MANAGER: Lazy<BarotraumaModManager> = Lazy::new(|| BarotraumaModManager::default());
