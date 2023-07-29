CREATE TABLE IF NOT EXISTS partners (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    name VARCHAR(20) NOT NULL,
    mail VARCHAR(20) NOT NULL,
    inserted_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX IF NOT EXISTS partners_mail ON partners (mail);

CREATE TABLE IF NOT EXISTS channels (
    id VARCHAR(20) NOT NULL PRIMARY KEY,
    is_active BOOLEAN NOT NULL,
    
    inserted_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS videos (
    -- passed by QueryParameter `v=xxx...`
    id VARCHAR(20) NOT NULL PRIMARY KEY,
    channel_id VARCHAR(20) NOT NULL,
    embed_code TEXT NOT NULL,

    inserted_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,

    CONSTRAINT fk_video_channel_id FOREIGN KEY (channel_id) REFERENCES channels (id)
);

CREATE TABLE IF NOT EXISTS affiliate_items (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    amazon_embed_code TEXT NOT NULL,
    asin VARCHAR(20) NOT NULL,
    is_valid BOOLEAN NOT NULL,
    partner_id BIGINT NOT NULL,

    inserted_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,

    CONSTRAINT fk_affiliate_items_partner_id FOREIGN KEY (partner_id) REFERENCES partners (id)
);

CREATE TABLE IF NOT EXISTS video_affiliate_item_belongings (
    video_id VARCHAR(20) NOT NULL,
    affiliate_item_id BIGINT NOT NULL,
    PRIMARY KEY (video_id, affiliate_item_id),
    
    inserted_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,

    CONSTRAINT fk_video_affiliate_item_belongings_video_id FOREIGN KEY (video_id) REFERENCES videos (id),
    CONSTRAINT fk_video_affiliate_item_belongings_item_id FOREIGN KEY (affiliate_item_id) REFERENCES affiliate_items (id)
);

CREATE TABLE IF NOT EXISTS channel_affiliate_item_belongings (
    channel_id VARCHAR(20) NOT NULL,
    affiliate_item_id BIGINT NOT NULL,
    PRIMARY KEY (channel_id, affiliate_item_id),

    inserted_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,

    CONSTRAINT fk_channel_affiliate_item_belongings_video_id FOREIGN KEY (channel_id) REFERENCES channels (id),
    CONSTRAINT fk_channel_affiliate_item_belongings_item_id FOREIGN KEY (affiliate_item_id) REFERENCES affiliate_items (id)
);