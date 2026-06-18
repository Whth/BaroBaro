use transmission::AttrRegistry;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reg = AttrRegistry::default();

    let (class_attrs, field_attrs) = reg
        .add_type_global_attr(
            "#[derive(serde::Deserialize, serde::Serialize)]\n#[serde(rename_all = \"camelCase\")]"
        )
        .add_field_attr(
            "WorkshopItem.published_file_id",
            "#[serde(alias = \"publishedfileid\",deserialize_with = \"crate::de::deserialize_u64\")]"
        )
        .add_field_attr(
            "WorkshopItem.creator",
            "#[serde(deserialize_with = \"crate::de::deserialize_u64\")]"
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
            "#[serde(alias = \"file_size\", deserialize_with = \"crate::de::deserialize_u64\")]"
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
