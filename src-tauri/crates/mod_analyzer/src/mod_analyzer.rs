use crate::mods::BarotraumaMod;
use constants::MOD_FILELIST_FILE;
use quick_xml::de::from_str;
use std::fs;
use std::path::Path;


impl BarotraumaMod {
    pub fn set_home_dir(&mut self, home_dir: String) -> &mut Self {
        self.home_dir = Some(home_dir);
        self
    }
    /// Creates a BarotraumaMod from an XML string.
    ///
    /// # Arguments
    /// * `s` - The XML string.
    ///
    /// # Returns
    /// A Result containing the parsed BarotraumaMod or an error.
    pub fn from_str(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mod_obj: BarotraumaMod = from_str(s)?;
        Ok(mod_obj)
    }

    /// Creates a BarotraumaMod from a file path.
    ///
    /// # Arguments
    /// * `path` - The path to the XML file.
    ///
    /// # Returns
    /// A Result containing the parsed BarotraumaMod or an error.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let xml_content = fs::read_to_string(path.as_ref())?;

        let parent_dir: &Path = path.as_ref().parent().ok_or_else(|| "Invalid path")?;
        Self::from_str(&xml_content).map(|mut mod_obj| {
            mod_obj.set_home_dir(parent_dir.to_string_lossy().to_string());
            mod_obj
        })
    }

    /// Creates a BarotraumaMod from a mod directory.
    ///
    /// # Arguments
    /// * `mod_dir` - The path to the mod directory.
    ///
    /// # Returns
    /// A Result containing the parsed BarotraumaMod or an error.
    pub fn from_mod_dir<P: AsRef<Path>>(mod_dir: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content_package_path = mod_dir.as_ref().join(MOD_FILELIST_FILE);
        Self::from_path(content_package_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;


    #[test]
    fn test_de_from_json() {
        let json = json!({
    "name": "Shipwrecks Extended",
    "modVersion": "1.0.17",
    "corePackage": false,
    "steamWorkshopId": 2095211492,
    "gameVersion": "1.0.8.0",
    "expectedHash": "D368F1974A26DBC851B0D53B9A66779A",
    "homeDir": "path\\mods\\Shipwrecks Extended",
    "size": null,
    "lastModified": null,
    "likes": null,
    "previewImage": "a image url",
    "subscribers": null,
    "creator": null,
    "description": null
});

        let mod_obj: BarotraumaMod = serde_json::from_value(json).expect("Failed to parse JSON");

        assert_eq!(mod_obj.name, "Shipwrecks Extended");
        assert_eq!(mod_obj.mod_version, "1.0.17");
        assert_eq!(mod_obj.preview_image, Some("a image url".to_string()))
    }

    fn get_test_xml() -> String {
        r#"<?xml version="1.0" encoding="utf-8"?>
<contentpackage name="BaroTraumatic" modversion="1.2.81" corepackage="False" steamworkshopid="2518816103" gameversion="1.9.8.0" expectedhash="9A54ACF2E7EBC95726A72AE966EF5F8D">
  <EventManagerSettings file="%ModDir%/Events/EventManagerSettings.xml" />
  <UIStyle file="%ModDir%/Text/style.xml" />
  <Missions file="%ModDir%/Events/Missions.xml" />
  <Afflictions file="%ModDir%/Afflictions/BTAffliction.xml" />
  <Item file="%ModDir%/Items/misc.xml" />
  <Item file="%ModDir%/Items/disposable_battery.xml" />
  <NPCSets file="%ModDir%/Events/BanditTweak.xml" />
  <RandomEvents file="%ModDir%/Events/randomcampaignevents.xml" />
  <Text file="%ModDir%/Text/English.xml" />
  <Text file="%ModDir%/Text/Russian.xml" />
  <Text file="%ModDir%/Text/LatinamericanSpanish.xml" />
  <Text file="%ModDir%/Text/SimplifiedChinese.xml" />
  <Other file="%ModDir%/Items/BTheadsets.png" />
  <Sounds file="%ModDir%/sounds.xml" />
  <RandomEvents file="%ModDir%/Events/randommissionevents.xml" />
</contentpackage>"#
            .to_string()
    }

    #[test]
    fn test_bool_deserialization() {
        let xml_true = r#"<contentpackage name="test" modversion="1" corepackage="True" steamworkshopid="" gameversion="" expectedhash="" />"#;
        let mod_obj = BarotraumaMod::from_str(xml_true).expect("Should parse 'True'");
        assert_eq!(mod_obj.core_package, true);

        let xml_false = r#"<contentpackage name="test" modversion="1" corepackage="False" steamworkshopid="" gameversion="" expectedhash="" />"#;
        let mod_obj = BarotraumaMod::from_str(xml_false).expect("Should parse 'False'");
        assert_eq!(mod_obj.core_package, false);

        let xml_mixed = r#"<contentpackage name="test" modversion="1" corepackage="false" steamworkshopid="" gameversion="" expectedhash="" />"#;
        let mod_obj = BarotraumaMod::from_str(xml_mixed).expect("Should parse 'false'");
        assert_eq!(mod_obj.core_package, false);
    }

    #[test]
    fn test_parse_content_package() {
        let xml = get_test_xml();
        let mod_obj = BarotraumaMod::from_str(&xml).expect("Failed to parse XML");

        // Basic metadata
        assert_eq!(mod_obj.name, "BaroTraumatic");
        assert_eq!(mod_obj.mod_version, "1.2.81");
        assert_eq!(mod_obj.core_package, false);
        assert_eq!(mod_obj.steam_workshop_id, 2518816103);
        assert_eq!(mod_obj.game_version, "1.9.8.0");
        assert_eq!(mod_obj.expected_hash, "9A54ACF2E7EBC95726A72AE966EF5F8D");
    }
}
