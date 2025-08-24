//! # Example
//!
//! ```no_run
//! use mod_analyzer::BaroConfig;
//!
//! let config = BaroConfig::from_file("config.xml")?;
//! println!("Core package: {}", config.core_package());
//!
//! for mod_entry in config.mods() {
//!     println!("[{}] {}", mod_entry.id(), mod_entry.path());
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use constants::BarotraumaHome;
use serde::Deserialize;
use std::path::Path;

/// Represents the parsed `config.xml` from Barotrauma.
#[derive(Debug, Deserialize)]
pub struct BaroConfig {
    #[serde(rename = "corepackage")]
    core_package: CorePackage,

    #[serde(rename = "contentpackages")]
    content_packages: ContentPackages,
}

/// The core content package (e.g., Vanilla.xml).
#[derive(Debug, Deserialize)]
struct CorePackage {
    #[serde(rename = "@path")]
    path: String,
}

/// Container for content packages.
#[derive(Debug, Deserialize)]
struct ContentPackages {
    #[serde(rename = "regularpackages")]
    regular_packages: RegularPackages,
}

/// Container for regular (mod) packages.
#[derive(Debug, Deserialize)]
struct RegularPackages {
    #[serde(rename = "package")]
    packages: Vec<Package>,
}

/// A single package entry in the config.
#[derive(Debug, Deserialize)]
struct Package {
    #[serde(rename = "@path")]
    path: String,
}

/// Represents an enabled mod under `LocalMods/`.
#[derive(Debug, Clone)]
pub struct ModEntry {
    path: String,
    id: usize,
}

impl ModEntry {
    /// Returns the full path of the mod, e.g., `LocalMods/123456789/filelist.xml`.
    pub fn path(&self) -> &str {
        &self.path
    }


    /// Returns the mod ID (folder name, typically the Steam Workshop ID).
    pub fn id(&self) -> usize {
        self.id
    }
}

impl BaroConfig {
    /// Parses `config.xml` from a file path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the XML file.
    ///
    /// # Returns
    ///
    /// A `Result<BaroConfig, E>`; returns error on failure.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let data = std::fs::read_to_string(path)?;
        Self::from_str(&data)
    }

    /// Parses XML from a byte slice.
    ///
    /// # Arguments
    ///
    /// * `xml` - Raw XML bytes.
    pub fn from_slice(xml: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let text = std::str::from_utf8(xml)?;
        Self::from_str(text)
    }

    /// Parses XML from a string.
    fn from_str(xml: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config: Self = quick_xml::de::from_str(xml)?;
        Ok(config)
    }

    /// Gets the core package path.
    ///
    /// # Returns
    ///
    /// Path to the core package, e.g., `"Content/ContentPackages/Vanilla.xml"`.
    pub fn core_package(&self) -> &str {
        &self.core_package.path
    }

    /// Gets the list of enabled mods under `LocalMods/`.
    ///
    /// Only includes packages with paths starting with `LocalMods/`.
    pub fn mods(&self) -> Vec<ModEntry> {
        self.content_packages
            .regular_packages
            .packages
            .iter()
            .map(|p| p.path.clone())
            .filter(|p| p.starts_with(format!("{}/", BarotraumaHome::MOD_DIR).as_str()))
            .filter_map(|path| {
                if let Some(id) = path.split('/').nth(1) &&
                    let Ok(id) = id.parse() {
                    Some(ModEntry {
                        path,
                        id,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_XML: &str = r#"
<config language="English">
  <graphicssettings width="1920" height="1080" />
  <corepackage path="Content/ContentPackages/Vanilla.xml" />
  <contentpackages>
    <regularpackages>
      <!-- EK Dockyard 重製版 -->
      <package path="LocalMods/3012187347/filelist.xml" />
      <!-- EK Utilities 重製版 -->
      <package path="LocalMods/3008781099/filelist.xml" />
      <!-- 外部 MOD -->
      <package path="Workshop/NotLocal/filelist.xml" />
      <!-- 正常 MOD -->
      <package path="LocalMods/2936760984/filelist.xml" />
    </regularpackages>
  </contentpackages>
  <player name="Whth" />
</config>
"#;

    #[test]
    fn test_parse_core_package() {
        let config = BaroConfig::from_str(TEST_XML).expect("Failed to parse");
        assert_eq!(config.core_package(), "Content/ContentPackages/Vanilla.xml");
    }

    #[test]
    fn test_parse_mods() {
        let config = BaroConfig::from_str(TEST_XML).expect("Failed to parse");
        let mods = config.mods();
        assert_eq!(mods.len(), 3);
        assert_eq!(mods[0].id(), 3012187347);
        assert_eq!(mods[0].path(), "LocalMods/3012187347/filelist.xml");
        assert_eq!(mods[1].id(), 3008781099);
        assert_eq!(mods[2].id(), 2936760984);
    }

    #[test]
    fn test_mod_entry_path_and_id() {
        let config = BaroConfig::from_str(TEST_XML).expect("Failed to parse");
        let mods = config.mods();
        for m in &mods {
            assert!(m.path().starts_with("LocalMods/"));
        }
    }

    #[test]
    fn test_from_slice() {
        let config = BaroConfig::from_slice(TEST_XML.as_bytes()).expect("Failed to parse");
        assert_eq!(config.core_package(), "Content/ContentPackages/Vanilla.xml");
        assert_eq!(config.mods().len(), 3);
    }
}