```mermaid
erDiagram
    partners {
        id Int PK
        name String
        mail String
    }

    channels {
        id String PK
        is_active bool
    }
    videos {
        id String PK
        channel_id Int FK
        embed_code String
    }

    channels ||--|{ videos : ""

    affiliate_items {
        id Int PK
        amazon_url Url
        is_valid bool
        partner_id Int FK
    }

    video_affiliate_items_belongings {
        affiliate_item_id Int FK
        video_id String FK
    }

    channel_affiliate_items_belongings {
        channel_id Int FK
        video_id String FK
    }

    affiliate_items ||..||  video_affiliate_items_belongings : "intermediate"
    video_affiliate_items_belongings ||..||  videos : "intermediate"

    affiliate_items ||..||  channel_affiliate_items_belongings : "intermediate"
    channel_affiliate_items_belongings ||..||  videos : "intermediate"

```
