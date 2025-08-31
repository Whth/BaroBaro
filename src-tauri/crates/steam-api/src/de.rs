use serde::Deserializer;

pub fn flexible_u64_deserializer<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Visitor;
    use std::fmt;

    struct U64Visitor;

    impl<'de> Visitor<'de> for U64Visitor {
        type Value = u64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or integer representing a u64")
        }

        fn visit_u64<E>(self, value: u64) -> Result<u64, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }

        fn visit_str<E>(self, s: &str) -> Result<u64, E>
        where
            E: serde::de::Error,
        {
            s.parse().map_err(E::custom)
        }

        fn visit_string<E>(self, s: String) -> Result<u64, E>
        where
            E: serde::de::Error,
        {
            self.visit_str(&s)
        }
    }

    deserializer.deserialize_any(U64Visitor)
}
