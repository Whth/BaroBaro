// Re-export shared deserializers from steam-api to avoid duplication.
// These are referenced via `crate::deserialize_bool` / `crate::deserialize_u64`
// in generated serde code from build.rs.
pub use steam_api::de::{deserialize_bool, deserialize_u64};
