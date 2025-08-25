use serde::de::Visitor;
use serde::Deserializer;
use std::fmt;

/// Custom deserializer for boolean values that supports "True"/"False" and "1"/"0" strings.

pub(crate) fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    struct BoolVisitor;

    impl Visitor<'_> for BoolVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a boolean, a string ('true', 'false', '1', '0'), or a number (0, 1)")
        }

        fn visit_bool<E>(self, v: bool) -> Result<bool, E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }

        fn visit_i64<E>(self, v: i64) -> Result<bool, E>
        where
            E: serde::de::Error,
        {
            match v {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(E::custom("Expected bool or 0/1")),
            }
        }

        fn visit_u64<E>(self, v: u64) -> Result<bool, E>
        where
            E: serde::de::Error,
        {
            match v {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(E::custom("Expected bool or 0/1")),
            }
        }

        // 支持 f64（比如 JSON 中写 1.0）
        fn visit_f64<E>(self, v: f64) -> Result<bool, E>
        where
            E: serde::de::Error,
        {
            if v == 1.0 {
                Ok(true)
            } else if v == 0.0 {
                Ok(false)
            } else {
                Err(E::custom("Expected 0.0 or 1.0"))
            }
        }

        fn visit_str<E>(self, s: &str) -> Result<bool, E>
        where
            E: serde::de::Error,
        {
            match s.trim().to_lowercase().as_str() {
                "true" | "1" => Ok(true),
                "false" | "0" => Ok(false),
                other => Err(E::custom(format!(
                    "Expected 'true', 'false', '1', '0', or boolean, got '{}'",
                    other
                ))),
            }
        }
    }

    deserializer.deserialize_any(BoolVisitor)
}

pub(crate) fn deserialize_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    struct U64Visitor;

    impl Visitor<'_> for U64Visitor {
        type Value = u64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or number")
        }

        fn visit_i64<E>(self, v: i64) -> Result<u64, E>
        where
            E: serde::de::Error,
        {
            if v >= 0 {
                Ok(v as u64)
            } else {
                Err(E::custom("u64 cannot be negative"))
            }
        }

        // 处理 JSON 整数: 123
        fn visit_u64<E>(self, v: u64) -> Result<u64, E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }

        // 处理 JSON 字符串: "123", "  ", "abc"
        fn visit_str<E>(self, s: &str) -> Result<u64, E>
        where
            E: serde::de::Error,
        {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                return Ok(0);
            }
            trimmed.parse::<u64>()
                .map_err(|_| E::custom("Invalid u64 value"))
        }

        // 处理已拥有的字符串（如从 borrow 转来）
        fn visit_string<E>(self, s: String) -> Result<u64, E>
        where
            E: serde::de::Error,
        {
            self.visit_str(&s)
        }
    }

    deserializer.deserialize_any(U64Visitor)
}