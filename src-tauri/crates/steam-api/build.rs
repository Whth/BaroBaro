use transmission::AttrRegistry;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reg = AttrRegistry::default();

    let (class_attrs, field_attrs) = reg
        .add_type_global_attr(
            "#[derive(serde::Deserialize, serde::Serialize)]\n#[serde(rename_all = \"camelCase\")]"
        )
        .add_field_attr(
            "WorkshopItem.published_file_id",
            "#[serde(alias = \"publishedfileid\",deserialize_with = \"crate::de::flexible_u64_deserializer\")]",
        )
        .add_field_attr(
            "WorkshopItem.creator",
            "#[serde(deserialize_with = \"crate::de::flexible_u64_deserializer\")]",
        )
        .add_field_attr(
            "WorkshopItem.creator_app_id",
            "#[serde(alias = \"creator_app_id\")]",
        )
        .add_field_attr(
            "WorkshopItem.consumer_app_id",
            "#[serde(alias = \"consumer_app_id\")]",
        )
        .add_field_attr(
            "WorkshopItem.file_size",
            "#[serde(alias = \"file_size\", deserialize_with = \"crate::de::flexible_u64_deserializer\")]",
        )
        .add_field_attr(
            "WorkshopItem.file_url",
            "#[serde(alias = \"file_url\")]",
        )
        .add_field_attr(
            "WorkshopItem.hcontent_file",
            "#[serde(alias = \"hcontent_file\")]",
        )
        .add_field_attr(
            "WorkshopItem.preview_url",
            "#[serde(alias = \"preview_url\")]",
        )
        .add_field_attr(
            "WorkshopItem.hcontent_preview",
            "#[serde(alias = \"hcontent_preview\")]",
        )
        .add_field_attr(
            "WorkshopItem.time_updated",
            "#[serde(alias = \"time_updated\")]",
        )
        .add_field_attr(
            "WorkshopItem.time_created",
            "#[serde(alias = \"time_created\")]",
        )

        .add_field_attr(
            "WorkshopItem.ban_reason",
            "#[serde(alias = \"ban_reason\")]",
        )

        .add_field_attr(
            "WorkshopItem.lifetime_favorited",
            "#[serde(alias = \"lifetime_favorited\")]",
        )


        .add_field_attr(
            "WorkshopItem.lifetime_subscriptions",
            "#[serde(alias = \"lifetime_subscriptions\")]",
        )

        .export();

    transmission::compile_proto_fine_grained("workshop", &class_attrs, &field_attrs)?;
    Ok(())
}

//*
//
// #[derive(Deserialize)]
// struct ApiResponse {
//     response: Response,
// }
//
// #[derive(Deserialize)]
// struct Response {
//     result: isize,
//     #[serde(rename = "resultcount")]
//     result_count: usize,
//     #[serde(rename = "publishedfiledetails")]
//     published_file_details: Vec<WorkshopItem>,
// }
//
// /// Represents a single Steam Workshop item.
// #[derive(Deserialize, Serialize, Debug, Clone)]
// pub struct WorkshopItem {
//     /// The unique 64-bit ID of the published file (now u64)
//     #[serde(
//         rename = "publishedfileid",
//         deserialize_with = "crate::de::flexible_u64_deserializer"
//     )]
//     pub published_file_id: u64,
//
//     /// Result code for this item (1 = success)
//     #[serde(rename = "result")]
//     pub result: i32,
//
//     /// Creator's Steam 64 ID
//     #[serde(rename = "creator", deserialize_with = "crate::de::flexible_u64_deserializer")]
//     pub creator: u64,
//
//     #[serde(rename = "creator_app_id")]
//     pub creator_app_id: u64,
//
//     #[serde(rename = "consumer_app_id")]
//     pub consumer_app_id: u64,
//
//     #[serde(rename = "filename")]
//     pub filename: String,
//
//     /// File size in bytes (as string — some values may exceed u64 in future?)
//     #[serde(rename = "file_size", deserialize_with = "crate::de::flexible_u64_deserializer")]
//     pub file_size: u64,
//
//     #[serde(rename = "file_url")]
//     pub file_url: String,
//
//     #[serde(rename = "hcontent_file")]
//     pub hcontent_file: String,
//
//     #[serde(rename = "preview_url")]
//     pub preview_url: String,
//
//     #[serde(rename = "hcontent_preview")]
//     pub hcontent_preview: String,
//
//     #[serde(rename = "title")]
//     pub title: String,
//
//     #[serde(rename = "description")]
//     pub description: String,
//
//     #[serde(rename = "time_created")]
//     pub time_created: u64,
//
//     #[serde(rename = "time_updated")]
//     pub time_updated: u64,
//
//     #[serde(rename = "visibility")]
//     pub visibility: i32,
//
//     #[serde(rename = "banned")]
//     pub banned: isize,
//
//     #[serde(rename = "ban_reason")]
//     pub ban_reason: String,
//
//     #[serde(rename = "subscriptions")]
//     pub subscriptions: u64,
//
//     #[serde(rename = "favorited")]
//     pub favorited: u64,
//
//     #[serde(rename = "lifetime_subscriptions")]
//     pub lifetime_subscriptions: u64,
//
//     #[serde(rename = "lifetime_favorited")]
//     pub lifetime_favorited: u64,
//
//     #[serde(rename = "views")]
//     pub views: u64,
//
//     #[serde(rename = "tags")]
//     pub tags: Vec<Tag>,
// }
//
// #[derive(Deserialize, Serialize, Debug, Clone)]
// pub struct Tag {
//     #[serde(rename = "tag")]
//     pub tag: String,
// }*//
