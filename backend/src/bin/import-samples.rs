use itertools::Itertools;
use sqlx::PgPool;
use usumurasaki::database::establish_connection;
use usumurasaki::models::{
    ApplicationResult, Channel, ChannelImporter, LibraryError, NewAffiliateItem, NewChannel,
    NewPartner, NewVideo, Partner, VideoAffilicateItemsBelonging, VideroAffilicateItemsImporter,
};
use usumurasaki::repository::{AffiliateItemRepo, ChannelRepo, PartnerRepo, VideoRepo};
const VIDEO_AFFILIATE_ITEMS: &[u8] = include_bytes!("../sample-data/video_affiliate_items.csv");

use csv::{ReaderBuilder, Trim};

pub fn csv_reader() -> ReaderBuilder {
    let mut builder = ReaderBuilder::new();
    builder
        .has_headers(true)
        .delimiter(b',')
        .flexible(true)
        .comment(Some(b'#'))
        .trim(Trim::None);
    builder
}

#[tokio::main]
async fn main() -> ApplicationResult<()> {
    let pool: sqlx::Pool<sqlx::Postgres> = establish_connection().await.unwrap();
    let partner = create_sample_partner_if_not_exists(pool.clone()).await;

    import_channels(pool.clone(), VIDEO_AFFILIATE_ITEMS, partner.id).await?;

    Ok(())
}

async fn import_channels(
    pool: PgPool,
    binary_from_csv: &[u8],
    partner_id: i64,
) -> ApplicationResult<()> {
    let importer: Vec<VideroAffilicateItemsImporter> = csv_reader()
        .trim(csv::Trim::All)
        .from_reader(binary_from_csv)
        .deserialize()
        .try_collect()
        .map_err(|_e| LibraryError::CSVCollect {
            msg: "failed to deserialize VideroAffilicateItemsImporter.".to_string(),
        })?;

    println!("importer: {:#?} ", importer);

    let importer = importer
        .into_iter()
        .sorted_by(|a, b| Ord::cmp(&b.channel_id, &a.channel_id))
        .sorted_by(|a, b| Ord::cmp(&b.video_id, &a.video_id));

    let mut new_channels = vec![];
    let mut new_videos = vec![];
    // (NewAffiliateItem, video_id, channel_id)
    let mut new_affilicate_items: Vec<(NewAffiliateItem, String, String)> = vec![];
    for imp in importer {
        new_channels.push(NewChannel {
            id: imp.channel_id.clone(),
        });
        new_videos.push(NewVideo {
            id: imp.video_id.clone(),
            channel_id: imp.channel_id.clone(),
            embed_code: imp.amazon_embed_code.clone(),
        });
        new_affilicate_items.push((
            NewAffiliateItem {
                amazon_url: imp.amazon_embed_code,
                is_valid: true,
                partner_id,
            },
            imp.video_id,
            imp.channel_id,
        ));
    }

    ChannelRepo::new(&pool).create(new_channels).await?;
    VideoRepo::new(&pool).create(new_videos).await?;
    let affiliate_items = new_affilicate_items
        .clone()
        .into_iter()
        .map(|v| v.0)
        .collect_vec();
    AffiliateItemRepo::new(&pool)
        .create(affiliate_items)
        .await?;
    // TODO CREATE BELONGINGS
    // TODO CREATE BELONGINGS

    Ok(())
}

async fn create_sample_partner_if_not_exists(pool: PgPool) -> Partner {
    let mut res = PartnerRepo::new(&pool)
        .create_or_update(vec![NewPartner {
            name: "test partner".to_string(),
            mail: "testmail-alxjdisaSLAMDsipbdsal@gmail.com".to_string(),
        }])
        .await
        .unwrap();
    let p = res.swap_remove(0);
    p
}
