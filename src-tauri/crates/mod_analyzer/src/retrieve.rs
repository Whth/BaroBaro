use crate::BarotraumaMod;
use logger::info;
use std::collections::HashMap;
use steam_api::{SteamWorkShopClient, WorkshopItem};
pub async fn retrieve_mod_metadata(
    mods: Vec<BarotraumaMod>,
    batch_size: usize,
    client: &SteamWorkShopClient,
) -> Result<Vec<BarotraumaMod>, Box<dyn std::error::Error>> {
    info!("Retrieving mod metadata for {} mods", mods.len());
    let retrieved: Vec<WorkshopItem> = client
        .get_items_batched(
            mods.iter()
                .map(|baro_mod| baro_mod.steam_workshop_id)
                .collect::<Vec<u64>>(),
            batch_size,
        )
        .await
        .map_err(|e| format!("{}, failed to retrieve mod metadata.", e))?;

    let mut mapping = mods
        .into_iter()
        .map(|baro_mod| (baro_mod.steam_workshop_id, baro_mod))
        .collect::<HashMap<u64, BarotraumaMod>>();
    retrieved.into_iter().for_each(|item| {
        if let Some(baro_mod) = mapping.get_mut(&item.published_file_id) {
            baro_mod.size = item.file_size.into();
            baro_mod.last_modified = item.time_updated.into();
            baro_mod.description = item.description.clone().into();
            baro_mod.preview_image = item.preview_url().to_string().into();
            baro_mod.subscribers = item.subscriptions.into();
            baro_mod.likes = item.favorited.into();
            baro_mod.creator = item.creator.into();
            baro_mod.tags = item.tags.into_iter().map(|tag| tag.tag).collect();
        }
    });

    Ok(mapping.into_values().collect::<Vec<BarotraumaMod>>())
}
