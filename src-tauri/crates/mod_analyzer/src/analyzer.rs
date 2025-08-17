use constants::MOD_FILELIST_FILE;
use quick_xml::de::from_str;
use serde::{Deserialize, Deserializer, de::MapAccess};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// 通用文件元素
#[derive(Debug, Clone, Deserialize)]
pub struct FileElement {
    #[serde(rename = "@file")]
    pub file: String,
}

// 自定义反序列化布尔值（支持 "True"/"False"）
fn deserialize_bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.to_lowercase().as_str() {
        "true" | "1" => Ok(true),
        "false" | "0" => Ok(false),
        other => Err(serde::de::Error::custom(format!(
            "Expected 'true', 'false', '1', or '0', got '{}'",
            other
        ))),
    }
}

// 我们不再定义固定字段的 ContentPackage
// 而是直接反序列化为动态结构
#[derive(Debug)]
pub struct BarotraumaMod {
    pub name: String,
    pub mod_version: String,
    pub core_package: bool,
    pub steam_workshop_id: String,
    pub game_version: String,
    pub expected_hash: String,
    // 所有标签名 -> 文件列表
    pub file_groups: HashMap<String, Vec<FileElement>>,
}

// 自定义反序列化器：处理任意子节点
impl<'de> Deserialize<'de> for BarotraumaMod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // 使用 `serde-xml-rs` 风格的 MapAccess 遍历所有字段
        let map = deserializer.deserialize_map(BarotraumaModVisitor)?;
        Ok(map)
    }
}

struct BarotraumaModVisitor;

impl<'de> serde::de::Visitor<'de> for BarotraumaModVisitor {
    type Value = BarotraumaMod;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a Barotrauma contentpackage XML")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        // 先提取属性
        let mut name = None;
        let mut mod_version = None;
        let mut core_package = None;
        let mut steam_workshop_id = None;
        let mut game_version = None;
        let mut expected_hash = None;

        // 用于收集所有标签
        let mut file_groups: HashMap<String, Vec<FileElement>> = HashMap::new();

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "@name" => name = Some(map.next_value()?),
                "@modversion" => mod_version = Some(map.next_value()?),
                "@corepackage" => {
                    // Use our custom deserializer for boolean values
                    let value: String = map.next_value()?;
                    let de = serde::de::value::StrDeserializer::new(&value);
                    core_package = Some(deserialize_bool_from_string(de)?);
                }
                "@steamworkshopid" => steam_workshop_id = Some(map.next_value()?),
                "@gameversion" => game_version = Some(map.next_value()?),
                "@expectedhash" => expected_hash = Some(map.next_value()?),
                _ => {
                    // 所有其他字段都当作标签处理
                    let values: Vec<FileElement> = map.next_value()?;
                    file_groups.entry(key).or_default().extend(values)
                }
            }
        }

        Ok(BarotraumaMod {
            name: name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
            mod_version: mod_version
                .ok_or_else(|| serde::de::Error::missing_field("modversion"))?,
            core_package: core_package
                .ok_or_else(|| serde::de::Error::missing_field("corepackage"))?,
            steam_workshop_id: steam_workshop_id
                .ok_or_else(|| serde::de::Error::missing_field("steamworkshopid"))?,
            game_version: game_version
                .ok_or_else(|| serde::de::Error::missing_field("gameversion"))?,
            expected_hash: expected_hash
                .ok_or_else(|| serde::de::Error::missing_field("expectedhash"))?,
            file_groups,
        })
    }
}

impl BarotraumaMod {
    pub fn from_str(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mod_obj: BarotraumaMod = from_str(s)?;
        Ok(mod_obj)
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let xml_content = fs::read_to_string(path)?;
        Self::from_str(&xml_content)
    }

    pub fn from_mod_dir<P: AsRef<Path>>(mod_dir: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content_package_path = mod_dir.as_ref().join(MOD_FILELIST_FILE);
        Self::from_path(content_package_path)
    }

    // 便捷方法：获取某类文件
    pub fn get_files(&self, tag: &str) -> Option<&Vec<FileElement>> {
        self.file_groups.get(tag)
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.file_groups.contains_key(tag)
    }

    pub fn tag_names(&self) -> Vec<String> {
        self.file_groups.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        // 基本元数据
        assert_eq!(mod_obj.name, "BaroTraumatic");
        assert_eq!(mod_obj.mod_version, "1.2.81");
        assert_eq!(mod_obj.core_package, false);
        assert_eq!(mod_obj.steam_workshop_id, "2518816103");
        assert_eq!(mod_obj.game_version, "1.9.8.0");
        assert_eq!(mod_obj.expected_hash, "9A54ACF2E7EBC95726A72AE966EF5F8D");

        // 文件组长度
        assert_eq!(mod_obj.get_files("Item").unwrap().len(), 2);
        assert_eq!(mod_obj.get_files("Text").unwrap().len(), 4);
        assert_eq!(mod_obj.get_files("RandomEvents").unwrap().len(), 2);
        assert_eq!(mod_obj.get_files("Sounds").unwrap().len(), 1);
        assert_eq!(mod_obj.get_files("Other").unwrap().len(), 1);
    }

    #[test]
    fn test_item_files() {
        let xml = get_test_xml();
        let mod_obj = BarotraumaMod::from_str(&xml).expect("Failed to parse XML");
        let items = mod_obj.get_files("Item").expect("No Item tag found");

        assert_eq!(items[0].file, "%ModDir%/Items/misc.xml");
        assert_eq!(items[1].file, "%ModDir%/Items/disposable_battery.xml");
    }

    #[test]
    fn test_text_files() {
        let xml = get_test_xml();
        let mod_obj = BarotraumaMod::from_str(&xml).expect("Failed to parse XML");
        let texts = mod_obj.get_files("Text").expect("No Text tag found");

        let expected = vec![
            "%ModDir%/Text/English.xml",
            "%ModDir%/Text/Russian.xml",
            "%ModDir%/Text/LatinamericanSpanish.xml",
            "%ModDir%/Text/SimplifiedChinese.xml",
        ];

        for (i, text) in texts.iter().enumerate() {
            assert_eq!(text.file, expected[i]);
        }
    }

    #[test]
    fn test_random_events_files() {
        let xml = get_test_xml();
        let mod_obj = BarotraumaMod::from_str(&xml).expect("Failed to parse XML");
        let events = mod_obj
            .get_files("RandomEvents")
            .expect("No RandomEvents tag found");

        assert_eq!(events[0].file, "%ModDir%/Events/randomcampaignevents.xml");
        assert_eq!(events[1].file, "%ModDir%/Events/randommissionevents.xml");
    }

    #[test]
    fn test_tag_names() {
        let xml = get_test_xml();
        let mod_obj = BarotraumaMod::from_str(&xml).expect("Failed to parse XML");

        let tags: std::collections::HashSet<_> = mod_obj.tag_names().into_iter().collect();
        let expected_tags: std::collections::HashSet<String> = [
            "EventManagerSettings",
            "UIStyle",
            "Missions",
            "Afflictions",
            "Item",
            "NPCSets",
            "RandomEvents",
            "Text",
            "Other",
            "Sounds",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(tags, expected_tags);
    }
}
