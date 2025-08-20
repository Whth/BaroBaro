use crate::mods::ModList;
use quick_xml::events::attributes::Attributes;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::{Reader, Writer};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
/// Represents a list of mods with their load order and configuration.
///
/// This struct holds information about a mod profile, including its name,
/// the base package it's built on, and the ordered list of local mods.
///
/// # Fields
///
/// * `name` - The name of the mod profile.
/// * `base_package` - The base package tag name (e.g., "Vanilla") - the first non-Local tag in the XML
/// * `local_mods` - A vector of local mods ordered by load order.
///
/// # Example
///
/// ```
/// use mod_analyzer::ModList;
///
/// let mod_list = ModList {
///     profile_name: "MyModProfile".to_string(),
///     base_package: "Vanilla".to_string(),
///     mods: vec![
///         "ModA".to_string(),
///         "ModB".to_string(),
///     ],
/// };
/// ```

impl ModList {
    /// Parses a ModList from an XML reader.
    ///
    /// This function reads XML data from a reader and constructs a ModList instance.
    /// The expected XML format has a root `<mods>` element with a `name` attribute,
    /// followed by a base package tag (e.g., `<Vanilla />`), and then zero or more
    /// `<Local>` elements with `name` attributes.
    ///
    /// # Arguments
    ///
    /// * `reader` - A type that implements `BufRead`, such as a file or string cursor
    ///
    /// # Returns
    ///
    /// * `Ok(ModList)` - If parsing was successful
    /// * `Err(...)` - If parsing failed due to malformed XML or missing required elements
    ///
    /// # Examples
    ///
    /// ```
    /// use mod_analyzer::ModList;
    /// use std::io::Cursor;
    ///
    /// let xml = r#"<mods name="Profile"><Vanilla /><Local name="ModA" /></mods>"#;
    /// let reader = Cursor::new(xml);
    /// let mod_list = ModList::from_xml(reader).unwrap();
    /// assert_eq!(mod_list.profile_name, "Profile");
    /// assert_eq!(mod_list.base_package, "Vanilla");
    /// assert_eq!(mod_list.mods, vec!["ModA"]);
    /// ```
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

                    // Get the name attribute of the root node
                    if tag_name == b"mods" && profile_name.is_none() {
                        profile_name = ModList::filter_name(e.attributes(), &reader);
                    }
                    // The first non-Local tag is the base_package
                    else if tag_name != b"Local" && base_package.is_none() {
                        base_package = Some(String::from_utf8_lossy(tag_name).to_string());
                    }
                    // For Local tags, parse the name attribute
                    else if tag_name == b"Local" {
                        ModList::filter_name(e.attributes(), &reader)
                            .into_iter()
                            .for_each(|name| mods.push(name));
                    }
                }
                Event::Eof => break,
                _ => (),
            }
            buf.clear();
        }

        Ok(ModList {
            profile_name: profile_name.ok_or_else(|| "missing profile name")?,
            base_package: base_package.ok_or_else(|| "missing base package (e.g. <Vanilla />)")?,
            mods,
        })
    }

    pub fn from_xml_path<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        ModList::from_xml(reader)
    }

    fn filter_name<R: BufRead>(attrs: Attributes, reader: &Reader<R>) -> Option<String> {
        attrs
            .filter_map(|a| a.ok())
            .filter(|a| a.key.as_ref() == b"name")
            .filter_map(|a| a.decode_and_unescape_value(reader.decoder()).ok())
            .last()
            .map(|s| s.to_string())
    }

    /// Serializes the ModList to XML format and writes it to a writer.
    ///
    /// This function generates well-formatted XML with 2-space indentation.
    /// The output includes the `<mods>` root element with the profile name,
    /// the base package as an empty element, and each mod as a `<Local>` element.
    ///
    /// # Arguments
    ///
    /// * `writer` - A type that implements `Write`, such as a file or buffer
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If writing was successful
    /// * `Err(...)` - If writing failed
    ///
    /// # Examples
    ///
    /// ```
    /// use mod_analyzer::ModList;
    /// use std::io::Cursor;
    ///
    /// let mod_list = ModList {
    ///     profile_name: "Profile".to_string(),
    ///     base_package: "Vanilla".to_string(),
    ///     mods: vec!["ModA".to_string()],
    /// };
    ///
    /// let mut buffer = Vec::new();
    /// mod_list.to_xml(&mut buffer).unwrap();
    /// let xml = String::from_utf8(buffer).unwrap();
    /// assert!(xml.contains(r#"<mods name="Profile">"#));
    /// assert!(xml.contains(r#"<Vanilla/>"#));
    /// assert!(xml.contains(r#"<Local name="ModA"/>"#));
    /// ```
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

    /// Converts the ModList to an XML string representation.
    ///
    /// This is a convenience method for converting the ModList to a string,
    /// which is useful for debugging or when you need to store the XML as a string.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The XML representation of the ModList
    /// * `Err(...)` - If serialization failed
    ///
    /// # Examples
    ///
    /// ```
    /// use mod_analyzer::ModList;
    ///
    /// let mod_list = ModList {
    ///     profile_name: "Profile".to_string(),
    ///     base_package: "Vanilla".to_string(),
    ///     mods: vec!["ModA".to_string()],
    /// };
    ///
    /// let xml = mod_list.to_string().unwrap();
    /// assert!(xml.contains(r#"<mods name="Profile">"#));
    /// ```
    pub fn to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        self.to_xml(&mut buf)?;
        Ok(String::from_utf8(buf)?)
    }

    /// Saves the ModList directly to a file at the specified path.
    ///
    /// This method creates or overwrites a file at the given path and writes
    /// the XML representation of the ModList to it.
    ///
    /// # Arguments
    ///
    /// * `path` - Anything that can be converted to a Path, such as a &str or PathBuf
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If saving was successful
    /// * `Err(...)` - If saving failed (e.g., due to file permissions)
    ///
    /// # Examples
    ///
    /// ```
    /// use mod_analyzer::ModList;
    /// use std::path::Path;
    ///
    /// let mod_list = ModList {
    ///     profile_name: "Profile".to_string(),
    ///     base_package: "Vanilla".to_string(),
    ///     mods: vec!["ModA".to_string()],
    /// };
    ///
    /// // This would save to a file named "mod_list.xml" in the current directory
    /// // mod_list.save_to_file("mod_list.xml").unwrap();
    /// ```
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

    /// Sample XML input used for testing the parsing functionality.
    ///
    /// This constant provides a complete example of a mod list XML structure,
    /// including a profile name, base package, and several local mods.
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

    /// Test parsing a full mod list XML.
    ///
    /// This test verifies that the `from_xml` function correctly parses a complete
    /// XML input, extracting the profile name, base package, and all mod names in order.
    #[test]
    fn test_parse_full() {
        let reader = Cursor::new(SAMPLE_XML);
        let order = ModList::from_xml(reader).expect("Failed to parse");

        assert_eq!(order.profile_name, "AG");
        assert_eq!(order.base_package, "Vanilla");
        assert_eq!(order.mods.len(), 5);
        assert_eq!(order.mods[0], "Immersive Sonar UI - A Real Sonar Add-On");
        assert_eq!(order.mods[4], "Animated arms");
    }

    /// Test parsing with a different base package.
    ///
    /// Ensures that the parser correctly identifies the first non-Local tag as the base package,
    /// even when it's not "Vanilla".
    #[test]
    fn test_base_package_case() {
        let xml = r#"<mods name="Test"><BEP /><Local name="Mod A" /></mods>"#;
        let order = ModList::from_xml(Cursor::new(xml)).unwrap();
        assert_eq!(order.base_package, "BEP");
    }

    /// Test parsing an empty mod list.
    ///
    /// Verifies that the parser handles cases where there are no Local mods,
    /// but still includes the required base package.
    #[test]
    fn test_empty_mods() {
        let xml = r#"<mods name="Empty"><Vanilla /></mods>"#;
        let order = ModList::from_xml(Cursor::new(xml)).unwrap();
        assert_eq!(order.profile_name, "Empty");
        assert_eq!(order.base_package, "Vanilla");
        assert!(order.mods.is_empty());
    }

    /// Test that parsing fails when the base package is missing.
    ///
    /// This test ensures that the parser returns an error when no base package tag
    /// is present in the XML, as it's a required element.
    #[test]
    #[should_panic(expected = "missing base package")]
    fn test_missing_base_package() {
        let xml = r#"<mods name="NoBase"><Local name="Mod A" /></mods>"#;
        let _ = ModList::from_xml(Cursor::new(xml)).unwrap();
    }

    /// Test serializing a `ModList` back to XML.
    ///
    /// This test checks that the `to_string` method produces valid XML output
    /// that matches the expected structure, including proper indentation and
    /// correct encoding of special characters.
    #[test]
    fn test_to_xml_string() {
        let order = ModList {
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
        // Check that key elements are present
        assert!(xml.contains(r#"<mods name="AG">"#));
        assert!(xml.contains(r#"<Vanilla/>"#));
        assert!(xml.contains(r#"<Local name="Immersive Sonar UI - A Real Sonar Add-On"/>"#));
        assert!(xml.contains(r#"<Local name="Animated arms"/>"#));
        assert!(xml.contains("  <")); // Indentation exists
    }

    /// Test saving and loading a `ModList` from a file.
    ///
    /// This test creates a `ModList`, saves it to a temporary file, then reads it back
    /// to ensure that the save and load operations are consistent and preserve data integrity.
    #[test]
    fn test_save_to_file() -> Result<(), Box<dyn std::error::Error>> {
        let order = ModList {
            profile_name: "TestSave".to_string(),
            base_package: "Vanilla".to_string(),
            mods: vec!["ModA".to_string(), "ModB".to_string()],
        };

        let temp_path = std::env::temp_dir().join("modloadorder_test.xml");
        order.save_to_file(&temp_path)?;

        // Read back and verify
        let loaded = ModList::from_xml(BufReader::new(std::fs::File::open(&temp_path)?))?;
        assert_eq!(loaded, order);

        // Clean up
        std::fs::remove_file(&temp_path)?;
        Ok(())
    }
}
