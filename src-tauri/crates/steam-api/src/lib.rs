//! # Steam Workshop Client
//!
//! An asynchronous Rust client for querying Steam Workshop item details
//! via the `ISteamRemoteStorage/GetPublishedFileDetails/v1/` API.
//!
//! This endpoint **requires HTTP POST**.
//!
//! ## Features
//! - Uses `u64` for item IDs (correct type for Steam's PublishedFileID)
//! - Fetch single or multiple items
//! - Proper error handling
//! - No unnecessary string allocations
//!
//! ## Example
//!
//! ```no_run
//! use steam_api::{SteamWorkShopClient, WorkshopItem};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = SteamWorkShopClient::new();
//!
//!     // Single item
//!     let item = client.get_item(3354525188).await?;
//!     println!("Title: {}", item.title);
//!
//!     // Multiple items
//!     let items = client.get_items(vec![3354525188, 1234567890]).await?;
//!     for item in items {
//!         println!("{}: {} views", item.title, item.views);
//!     }
//!
//!     Ok(())
//! }
//! ```

use futures::future::try_join_all;
use reqwest;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};
use std::iter::once;
use thiserror::Error;

/// Main client for querying Steam Workshop API.
#[derive(Debug, Clone)]
pub struct SteamWorkShopClient {
    client: reqwest::Client,
    endpoint: String,
}

impl Default for SteamWorkShopClient {
    fn default() -> Self {
        let client = reqwest::Client::new();
        SteamWorkShopClient {
            client,
            endpoint: Self::DEFAULT_ENDPOINT.to_string(),
        }
    }
}

impl SteamWorkShopClient {
    const DEFAULT_ENDPOINT: &'static str = "https://api.steampowered.com/ISteamRemoteStorage/GetPublishedFileDetails/v1/";
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates a new `WorkshopClient`.
    pub fn from_endpoint(endpoint: String) -> Self {
        let client = reqwest::Client::new();
        SteamWorkShopClient {
            client,
            endpoint,
        }
    }


    /// Fetch a single Workshop item by its published file ID (u64).
    ///
    /// # Arguments
    ///
    /// * `item_id` - The 64-bit numeric ID of the Workshop item (e.g. `3354525188u64`)
    ///
    /// # Returns
    ///
    /// Returns `Ok(WorkshopItem)` on success.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use steam_api::SteamWorkShopClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SteamWorkShopClient::new();
    /// let item = client.get_item(3354525188).await?;
    /// # Ok(()) }
    /// ```
    pub async fn get_item(&self, item_id: u64) -> Result<WorkshopItem, Error> {
        self.get_items(vec![item_id])
            .await?
            .into_iter()
            .next()
            .ok_or(Error::EmptyResponse)
    }

    /// Fetch multiple Workshop items by their IDs (as `u64`).
    ///
    /// Maximum: 100 items per request (Steam API limit).
    ///
    /// # Arguments
    ///
    /// * `item_ids` - A vector of `u64` Workshop item IDs
    ///
    /// # Returns
    ///
    /// Returns a `Vec<WorkshopItem>` of successfully returned items.
    ///
    /// ⚠️ Steam may return fewer items than requested (e.g., deleted, private, or banned items).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use steam_api::SteamWorkShopClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SteamWorkShopClient::new();
    /// let items = client.get_items(vec![3354525188, 1234567890]).await?;
    /// # Ok(()) }
    /// ```
    pub async fn get_items(&self, item_ids: Vec<u64>) -> Result<Vec<WorkshopItem>, Error> {
        if item_ids.is_empty() {
            return Ok(vec![]);
        }

        let form_data =

            once(("itemcount".to_string(), Value::from(item_ids.len())))
                .chain(
                    item_ids.iter()
                        .enumerate()
                        .map(|(idx, &id)| {
                            (format!("publishedfileids[{}]", idx), Value::from(id))
                        })
                )
                .collect::<Map<String, Value>>();


        let response: ApiResponse = self
            .client
            .post(self.endpoint.as_str())
            .form(&form_data)
            .send()
            .await?
            .json()
            .await?;

        if response.response.result != 1 || response.response.result_count != item_ids.len() {
            return Err(Error::ApiFailure(response.response.result));
        }

        let items = response
            .response
            .published_file_details
            .into_iter()
            .filter(|item| item.result == 1)
            .collect();

        Ok(items)
    }
    /// Fetch multiple Workshop items by their IDs (as `u64`) in batches.
    ///
    /// This method splits the provided item IDs into batches of the specified size
    /// and executes the requests concurrently. This is useful when you have more
    /// than 100 items to fetch, as the Steam API limits each request to 100 items.
    ///
    /// # Arguments
    ///
    /// * `item_ids` - A vector of `u64` Workshop item IDs
    /// * `batch_size` - The maximum number of items per batch (0 means unlimited)
    ///
    /// # Returns
    ///
    /// Returns a `Vec<WorkshopItem>` containing all successfully fetched items.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - `batch_size` is zero
    /// - Any individual batch request fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use steam_api::SteamWorkShopClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SteamWorkShopClient::new();
    /// let items = client.get_items_batched(vec![3354525188, 1234567890], 50).await?;
    /// # Ok(()) }
    /// ```
    pub async fn get_items_batched(
        &self,
        item_ids: Vec<u64>,
        batch_size: usize,
    ) -> Result<Vec<WorkshopItem>, Error> {
        if batch_size == 0 {
            return self.get_items(item_ids).await;
        }

        if item_ids.is_empty() {
            return Ok(vec![]);
        }

        // Split item_ids into multiple batches
        let batches = item_ids
            .chunks(batch_size)
            .map(|chunk| chunk.to_vec())
            .map(|batch| self.get_items(batch))
            .collect::<Vec<_>>();

        // Execute all batches concurrently
        let results = try_join_all(batches).await?;

        // Merge results from all batches
        let mut all_items = Vec::new();
        for items in results {
            all_items.extend(items);
        }

        Ok(all_items)
    }
}

// ===================================
// Data Structures
// ===================================

#[derive(Deserialize)]
struct ApiResponse {
    response: Response,
}

#[derive(Deserialize)]
struct Response {
    result: isize,
    #[serde(rename = "resultcount")]
    result_count: usize,
    #[serde(rename = "publishedfiledetails")]
    published_file_details: Vec<WorkshopItem>,
}

fn flexible_u64_deserializer<'de, D>(deserializer: D) -> Result<u64, D::Error>
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

/// Represents a single Steam Workshop item.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WorkshopItem {
    /// The unique 64-bit ID of the published file (now u64)
    #[serde(rename = "publishedfileid", deserialize_with = "flexible_u64_deserializer")]
    pub published_file_id: u64,

    /// Result code for this item (1 = success)
    #[serde(rename = "result")]
    pub result: i32,

    /// Creator's Steam 64 ID
    #[serde(rename = "creator", deserialize_with = "flexible_u64_deserializer")]
    pub creator: u64,

    #[serde(rename = "creator_app_id")]
    pub creator_app_id: u64,

    #[serde(rename = "consumer_app_id")]
    pub consumer_app_id: u64,

    #[serde(rename = "filename")]
    pub filename: String,

    /// File size in bytes (as string — some values may exceed u64 in future?)
    #[serde(rename = "file_size", deserialize_with = "flexible_u64_deserializer")]
    pub file_size: u64,

    #[serde(rename = "file_url")]
    pub file_url: String,

    #[serde(rename = "hcontent_file")]
    pub hcontent_file: String,

    #[serde(rename = "preview_url")]
    pub preview_url: String,

    #[serde(rename = "hcontent_preview")]
    pub hcontent_preview: String,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "time_created")]
    pub time_created: u64,

    #[serde(rename = "time_updated")]
    pub time_updated: u64,

    #[serde(rename = "visibility")]
    pub visibility: i32,

    #[serde(rename = "banned")]
    pub banned: isize,

    #[serde(rename = "ban_reason")]
    pub ban_reason: String,

    #[serde(rename = "subscriptions")]
    pub subscriptions: u64,

    #[serde(rename = "favorited")]
    pub favorited: u64,

    #[serde(rename = "lifetime_subscriptions")]
    pub lifetime_subscriptions: u64,

    #[serde(rename = "lifetime_favorited")]
    pub lifetime_favorited: u64,

    #[serde(rename = "views")]
    pub views: u64,

    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tag {
    #[serde(rename = "tag")]
    pub tag: String,
}

// ===================================
// Error Types
// ===================================

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("API call failed with result code {0}")]
    ApiFailure(isize),

    #[error("no items returned in response")]
    EmptyResponse,

}

// ===================================
// Convenience Methods
// ===================================

impl WorkshopItem {
    /// Returns trimmed preview URL.
    pub fn preview_url(&self) -> &str {
        self.preview_url.trim()
    }

    /// Parses file size into bytes (`u64`). Returns 0 if parse fails.
    pub fn file_size_bytes(&self) -> u64 {
        self.file_size
    }

    /// Whether the item is banned.
    pub fn is_banned(&self) -> bool {
        self.banned == 1
    }

    /// Returns a vector of tag names.
    pub fn tag_names(&self) -> Vec<&str> {
        self.tags.iter().map(|t| t.tag.as_str()).collect()
    }
}

// Optional: Implement Default for easier testing
impl Default for WorkshopItem {
    fn default() -> Self {
        Self {
            published_file_id: 0,
            result: 1,
            creator: 0,
            creator_app_id: 0,
            consumer_app_id: 0,
            filename: String::new(),
            file_size: 0,
            file_url: String::new(),
            hcontent_file: String::new(),
            preview_url: String::new(),
            hcontent_preview: String::new(),
            title: String::new(),
            description: String::new(),
            time_created: 0,
            time_updated: 0,
            visibility: 0,
            banned: 0,
            ban_reason: String::new(),
            subscriptions: 0,
            favorited: 0,
            lifetime_subscriptions: 0,
            lifetime_favorited: 0,
            views: 0,
            tags: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Server, ServerGuard};
    use serde_json::json;

    const PATH: &str = "/ISteamRemoteStorage/GetPublishedFileDetails/v1/";

    fn mount_path(host: String) -> String {
        format!("{host}{PATH}")
    }
    /// Helper to create a mock Steam API response for one or more items
    #[inline]
    async fn setup_mock_response(server: &mut ServerGuard, body: Value) -> String {
        server.mock("POST", PATH)
            .with_status(200)
            .with_header("Content-Type", "application/json")
            .with_body(body.to_string())
            .create();
        mount_path(server.url())
    }

    #[tokio::test]
    async fn test_get_single_item_success() {
        // Arrange
        let response_json = json!({
            "response": {
                "result": 1,
                "resultcount": 1,
                "publishedfiledetails": [{
                    "publishedfileid": 3354525188_u64,
                    "result": 1,
                    "creator": "123456789",
                    "creator_app_id": 1000,
                    "consumer_app_id": 1000,
                    "filename": "mod.zip",
                    "file_size": "1048576",
                    "file_url": "",
                    "hcontent_file": "abc123",
                    "preview_url": "https://example.com/preview.jpg",
                    "hcontent_preview": "def456",
                    "title": "Awesome Mod",
                    "description": "A great mod.",
                    "time_created": 1670000000,
                    "time_updated": 1670000001,
                    "visibility": 0,
                    "banned": 0,
                    "ban_reason": "",
                    "subscriptions": 100,
                    "favorited": 50,
                    "lifetime_subscriptions": 150,
                    "lifetime_favorited": 75,
                    "views": 1000,
                    "tags": [
                        { "tag": "Survival" },
                        { "tag": "Mods" }
                    ]
                }]
            }
        });

        let mut server = Server::new_async().await;
        let url = setup_mock_response(&mut server, response_json).await;

        // Act
        let client = SteamWorkShopClient::from_endpoint(url);
        let item = client.get_item(3354525188).await;

        // Assert
        let item = item.expect("Expected successful item fetch");
        assert_eq!(item.published_file_id, 3354525188);
        assert_eq!(item.title, "Awesome Mod");
        assert_eq!(item.views, 1000);
        assert_eq!(item.tag_names(), vec!["Survival", "Mods"]);
        assert!(!item.is_banned());
        assert_eq!(item.file_size_bytes(), 1_048_576);
    }

    #[tokio::test]
    async fn test_get_multiple_items_success() {
        // Arrange
        let response_json = json!({
            "response": {
                "result": 1,
                "resultcount": 2,
                "publishedfiledetails": [
                    {
                        "publishedfileid": "3354525188",
                        "result": 1,
                        "creator": "123456789",
                        "creator_app_id": 1000,
                        "consumer_app_id": 1000,
                        "title": "Mod One",
                        "file_size": 512000,
                        "views": 500,
                        "time_created": 1670000000,
                        "time_updated": 1670000001,
                        "visibility": 0,
                        "banned": 0,
                        "subscriptions": 90,
                        "favorited": 40,
                        "lifetime_subscriptions": 130,
                        "lifetime_favorited": 60,
                        "description": "",
                        "filename": "",
                        "file_url": "",
                        "hcontent_file": "",
                        "preview_url": "",
                        "hcontent_preview": "",
                        "ban_reason": "",
                        "tags": []
                    },
                    {
                        "publishedfileid": "1234567890",
                        "result": 1,
                        "creator": "987654321",
                        "creator_app_id": 1000,
                        "consumer_app_id": 1000,
                        "title": "Mod Two",
                        "file_size": "2048000",
                        "views": 800,
                        "time_created": 1670000002,
                        "time_updated": 1670000003,
                        "visibility": 0,
                        "banned": 0,
                        "subscriptions": 120,
                        "favorited": 60,
                        "lifetime_subscriptions": 180,
                        "lifetime_favorited": 90,
                        "description": "",
                        "filename": "",
                        "file_url": "",
                        "hcontent_file": "",
                        "preview_url": "",
                        "hcontent_preview": "",
                        "ban_reason": "",
                        "tags": [{ "tag": "PvE" }]
                    }
                ]
            }
        });

        let mut server = Server::new_async().await;

        let url = setup_mock_response(&mut server, response_json).await;

        // Act
        let client = SteamWorkShopClient::from_endpoint(url);
        let items = client.get_items(vec![3354525188, 1234567890]).await;

        // Assert
        let items = items.expect("Expected successful items fetch");
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].title, "Mod One");
        assert_eq!(items[1].title, "Mod Two");
        assert_eq!(items[1].tag_names(), vec!["PvE"]);
    }

    #[tokio::test]
    async fn test_get_items_empty_list_returns_empty() {
        // Arrange: No mock needed since we return early
        let client = SteamWorkShopClient::new();

        // Act
        let items = client.get_items(vec![]).await;

        // Assert
        let items = items.expect("Expected OK with empty vec");
        assert!(items.is_empty());
    }

    #[tokio::test]
    async fn test_get_items_api_failure() {
        // Arrange
        let response_json = json!({
            "response": {
                "result": 8, // 8 = Invalid parameter or other failure
                "resultcount": 0,
                "publishedfiledetails": []
            }
        });

        let mut server = Server::new_async().await;


        let url = setup_mock_response(&mut server, response_json).await;
        let client = SteamWorkShopClient::from_endpoint(url);
        let result = client.get_items(vec![123]).await;

        // Assert
        assert!(result.is_err());
        if let Err(Error::ApiFailure(code)) = result {
            assert_eq!(code, 8);
        } else {
            panic!("Expected ApiFailure(8)");
        }
    }

    #[tokio::test]
    async fn test_get_item_not_found_in_response() {
        // Arrange: API returns success, but item list is empty
        let response_json = json!({
            "response": {
                "result": 1,
                "resultcount": 0,
                "publishedfiledetails": []
            }
        });

        let mut server = Server::new_async().await;
        let url = setup_mock_response(&mut server, response_json).await;
        let client = SteamWorkShopClient::from_endpoint(url);


        let result = client.get_item(123).await;

        // Assert
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::ApiFailure(1))));
    }

    #[tokio::test]
    async fn test_item_banned_flag() {
        // Arrange
        let response_json = json!({
            "response": {
                "result": 1,
                "resultcount": 1,
                "publishedfiledetails": [{
                    "publishedfileid": "123",
                    "result": 1,
                    "creator": "123",
                    "creator_app_id": 1000,
                    "consumer_app_id": 1000,
                    "filename": "",
                    "file_size": "0",
                    "file_url": "",
                    "hcontent_file": "",
                    "preview_url": "  https://img.com/preview.jpg  ",
                    "hcontent_preview": "",
                    "title": "Banned Mod",
                    "description": "",
                    "time_created": 0,
                    "time_updated": 0,
                    "visibility": 0,
                    "banned": 1,
                    "ban_reason": "Inappropriate content",
                    "subscriptions": 0,
                    "favorited": 0,
                    "lifetime_subscriptions": 0,
                    "lifetime_favorited": 0,
                    "views": 0,
                    "tags": []
                }]
            }
        });

        let mut server = Server::new_async().await;

        let url = setup_mock_response(&mut server, response_json).await;
        let client = SteamWorkShopClient::from_endpoint(url);
        let item = client.get_item(123).await.expect("Should not fail");

        // Assert
        assert!(item.is_banned());
        assert_eq!(item.ban_reason, "Inappropriate content");
        assert_eq!(item.preview_url(), "https://img.com/preview.jpg");
    }

    #[tokio::test]
    async fn test_form_data_structure() {
        // This test ensures we send correct form data to Steam API

        let mut server = Server::new_async().await;
        let form_captured = server.mock("POST", PATH)
            .match_body("itemcount=2&publishedfileids%5B0%5D=123&publishedfileids%5B1%5D=456")
            .with_status(200)
            .with_header("Content-Type", "application/json")
            .with_body(
                json!({
                    "response": {
                        "result": 1,
                        "resultcount": 0,
                        "publishedfiledetails": []
                    }
                })
                    .to_string(),
            )
            .expect_at_least(1)
            .create();

        let client = SteamWorkShopClient::from_endpoint(mount_path(server.url()));
        let _ = client.get_items(vec![123, 456]).await;

        form_captured.assert();
    }
}