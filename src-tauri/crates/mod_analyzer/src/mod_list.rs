use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::{Reader, Writer};
use std::io::{BufRead, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModLoadOrder {
    pub profile_name: String, // mods name="..."
    pub base_package: String, // 第一个非-Local 标签名，如 "Vanilla"
    pub mods: Vec<String>,    // 所有 Local name 的顺序列表
}

impl ModLoadOrder {
    pub fn from_xml<R: BufRead>(reader: R) -> Result<Self, Box<dyn std::error::Error>> {
        let mut reader = Reader::from_reader(reader);
        reader.config_mut().trim_text(true);

        let mut profile_name: Option<String> = None;
        let mut base_package: Option<String> = None;
        let mut mods: Vec<String> = Vec::new();
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(ref e) | Event::Empty(ref e) => {
                    let tag_name = e.name().0;

                    // 获取根节点的 name 属性
                    if tag_name == b"mods" && profile_name.is_none() {
                        for attr in e.attributes() {
                            let attr = attr?;
                            if attr.key.as_ref() == b"name" {
                                profile_name = Some(
                                    attr.decode_and_unescape_value(reader.decoder())?
                                        .into_owned(),
                                );
                                break;
                            }
                        }
                    }
                    // 第一个非-Local 标签即为 base_package
                    else if tag_name != b"Local" && base_package.is_none() {
                        base_package = Some(String::from_utf8_lossy(tag_name).to_string());
                    }
                    // Local 标签则解析 name 属性
                    else if tag_name == b"Local" {
                        for attr in e.attributes() {
                            let attr = attr?;
                            if attr.key.as_ref() == b"name" {
                                let name = attr.decode_and_unescape_value(reader.decoder())?;
                                mods.push(name.into_owned());
                                break;
                            }
                        }
                    }
                }
                Event::Eof => break,
                _ => (),
            }
            buf.clear();
        }

        Ok(ModLoadOrder {
            profile_name: profile_name.ok_or_else(|| "missing profile name")?,
            base_package: base_package.ok_or_else(|| "missing base package (e.g. <Vanilla />)")?,
            mods,
        })
    }

    /// 将 ModLoadOrder 写入 Writer，生成格式化的 XML
    pub fn to_xml<W: Write>(&self, writer: W) -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = Writer::new_with_indent(writer, b' ', 2); // 2-space indent

        // <mods name="...">
        let mut root_elem = BytesStart::new("mods");
        root_elem.push_attribute(("name", self.profile_name.as_str()));
        writer.write_event(Event::Start(root_elem))?;

        // 换行 + 缩进
        writer.write_event(Event::Text(BytesText::new("\n  ")))?;

        // <base_package />
        {
            let elem = BytesStart::new(self.base_package.as_str());
            writer.write_event(Event::Empty(elem))?;
            writer.write_event(Event::Text(BytesText::new("\n  ")))?;
        }

        // <Local name="..." /> for each mod
        for mod_name in &self.mods {
            let mut elem = BytesStart::new("Local");
            elem.push_attribute(("name", mod_name.as_str()));
            writer.write_event(Event::Empty(elem))?;
            writer.write_event(Event::Text(BytesText::new("\n  ")))?;
        }

        // </mods>
        writer.write_event(Event::End(BytesEnd::new("mods")))?;
        writer.write_event(Event::Eof)?; // This ensures final newline

        Ok(())
    }
    /// 将 ModLoadOrder 转换为 XML 字符串（用于调试或保存到文件）
    pub fn to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        self.to_xml(&mut buf)?;
        Ok(String::from_utf8(buf)?)
    }

    /// 将 ModLoadOrder 直接写入文件路径
    pub fn save_to_file<P: AsRef<std::path::Path>>(
        &self,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        self.to_xml(writer)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Cursor};

    const SAMPLE_XML: &str = r#"
<mods name="AG">
  <Vanilla />
  <Local name="Immersive Sonar UI - A Real Sonar Add-On" />
  <Local name="Real Sonar" />
  <Local name="木萌BaldFix" />
  <Local name="[EA-HI]Animated Barotrauma Lite" />
  <Local name="Animated arms" />
</mods>
"#;

    #[test]
    fn test_parse_full() {
        let reader = Cursor::new(SAMPLE_XML);
        let order = ModLoadOrder::from_xml(reader).expect("Failed to parse");

        assert_eq!(order.profile_name, "AG");
        assert_eq!(order.base_package, "Vanilla");
        assert_eq!(order.mods.len(), 5);
        assert_eq!(order.mods[0], "Immersive Sonar UI - A Real Sonar Add-On");
        assert_eq!(order.mods[4], "Animated arms");
    }

    #[test]
    fn test_base_package_case() {
        let xml = r#"<mods name="Test"><BEP /><Local name="Mod A" /></mods>"#;
        let order = ModLoadOrder::from_xml(Cursor::new(xml)).unwrap();
        assert_eq!(order.base_package, "BEP");
    }

    #[test]
    fn test_empty_mods() {
        let xml = r#"<mods name="Empty"><Vanilla /></mods>"#;
        let order = ModLoadOrder::from_xml(Cursor::new(xml)).unwrap();
        assert_eq!(order.profile_name, "Empty");
        assert_eq!(order.base_package, "Vanilla");
        assert!(order.mods.is_empty());
    }

    #[test]
    #[should_panic(expected = "missing base package")]
    fn test_missing_base_package() {
        let xml = r#"<mods name="NoBase"><Local name="Mod A" /></mods>"#;
        let _ = ModLoadOrder::from_xml(Cursor::new(xml)).unwrap();
    }

    #[test]
    fn test_to_xml_string() {
        let order = ModLoadOrder {
            profile_name: "AG".to_string(),
            base_package: "Vanilla".to_string(),
            mods: vec![
                "Immersive Sonar UI - A Real Sonar Add-On".to_string(),
                "Real Sonar".to_string(),
                "木萌BaldFix".to_string(),
                "[EA-HI]Animated Baroterra Lite".to_string(),
                "Animated arms".to_string(),
            ],
        };

        let xml = order.to_string().expect("Failed to serialize");
        println!("{}", xml);
        // 检查关键内容是否包含
        assert!(xml.contains(r#"<mods name="AG">"#));
        assert!(xml.contains(r#"<Vanilla/>"#));
        assert!(xml.contains(r#"<Local name="Immersive Sonar UI - A Real Sonar Add-On"/>"#));
        assert!(xml.contains(r#"<Local name="Animated arms"/>"#));
        assert!(xml.contains("  <")); // 缩进存在
    }

    #[test]
    fn test_save_to_file() -> Result<(), Box<dyn std::error::Error>> {
        let order = ModLoadOrder {
            profile_name: "TestSave".to_string(),
            base_package: "Vanilla".to_string(),
            mods: vec!["ModA".to_string(), "ModB".to_string()],
        };

        let temp_path = std::env::temp_dir().join("modloadorder_test.xml");
        order.save_to_file(&temp_path)?;

        // 读回验证
        let loaded = ModLoadOrder::from_xml(BufReader::new(std::fs::File::open(&temp_path)?))?;
        assert_eq!(loaded, order);

        // 清理
        std::fs::remove_file(&temp_path)?;
        Ok(())
    }
}
