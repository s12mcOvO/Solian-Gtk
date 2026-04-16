use crate::core::models::{SnSticker, SnStickerPack};
use crate::core::network::ApiClient;
use anyhow::Result;
use std::sync::Arc;

pub struct StickerService {
    client: Arc<ApiClient>,
}

impl StickerService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn get_sticker_packs(&self) -> Result<Vec<SnStickerPack>> {
        let packs: Vec<SnStickerPack> = self.client.get("/sphere/stickers").await?.json().await?;
        Ok(packs)
    }

    pub async fn get_sticker_pack(&self, pack_id: &str) -> Result<SnStickerPack> {
        let pack: SnStickerPack = self.client.get(&format!("/sphere/stickers/{}", pack_id)).await?.json().await?;
        Ok(pack)
    }

    pub async fn get_stickers(&self, pack_id: &str) -> Result<Vec<SnSticker>> {
        let stickers: Vec<SnSticker> = self.client.get(&format!("/sphere/stickers/{}/content", pack_id)).await?.json().await?;
        Ok(stickers)
    }

    pub async fn get_my_sticker_packs(&self) -> Result<Vec<SnStickerPack>> {
        let packs: Vec<SnStickerPack> = self.client.get("/sphere/stickers/me").await?.json().await?;
        Ok(packs)
    }

    pub async fn create_sticker_pack(&self, name: &str, description: &str) -> Result<SnStickerPack> {
        #[derive(serde::Serialize)]
        struct CreateRequest<'a> {
            name: &'a str,
            description: &'a str,
        }
        let pack: SnStickerPack = self.client.post("/sphere/stickers", &CreateRequest { name, description }).await?;
        Ok(pack)
    }

    pub async fn purchase_sticker_pack(&self, pack_id: &str) -> Result<()> {
        self.client.post::<(), ()>(&format!("/sphere/stickers/{}/own", pack_id), &()).await
    }
}