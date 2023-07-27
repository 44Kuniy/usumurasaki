use super::{ChannelId, VideoId};

pub struct ChannelImporter {
    pub id: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct VideoAffiliateItemsImporter {
    pub channel_id: ChannelId,
    pub video_id: VideoId,
    pub amazon_embed_code: String,
}
