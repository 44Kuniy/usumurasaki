pub struct ChannelImporter {
    pub id: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct VideroAffilicateItemsImporter {
    pub channel_id: String,
    pub video_id: String,
    pub amazon_embed_code: String,
}
