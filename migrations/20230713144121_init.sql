CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    name VARCHAR(20) NOT NULL,
    sub_name VARCHAR(20),
    inserted_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS channels (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    code VARCHAR(20) NOT NULL,
    is_active BOOLEAN NOT NULL,
    -- MainChannel = 0 , SubChannel = 1, ...
    rank SMALLINT NOT NULL,
    inserted_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    CONSTRAINT fk_channels_user_id FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE IF NOT EXISTS videos (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    channel_id BIGINT NOT NULL,
    uri TEXT NOT NULL,
    CONSTRAINT fk_video_channel_id FOREIGN KEY (channel_id) REFERENCES channels (id)
);

CREATE TABLE IF NOT EXISTS video_user_belongings (
    video_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    PRIMARY KEY (video_id, user_id),
    CONSTRAINT fk_video_user_belongings_video_id FOREIGN KEY (video_id) REFERENCES videos (id),
    CONSTRAINT fk_video_user_belongings_user_id FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE IF NOT EXISTS affiliate_items (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    r_user_id BIGINT NOT NULL,
    amazon_uri TEXT,
    rauten_uri TEXT,
    is_valid BOOLEAN NOT NULL,
    CONSTRAINT fk_affiliate_items_user_id FOREIGN KEY (r_user_id) REFERENCES users (id)
);

CREATE TABLE IF NOT EXISTS video_affiliate_item_belongings (
    video_id BIGINT NOT NULL,
    affiliate_item_id BIGINT NOT NULL,
    PRIMARY KEY (video_id, affiliate_item_id),
    CONSTRAINT fk_video_affiliate_item_belongings_video_id FOREIGN KEY (video_id) REFERENCES videos (id),
    CONSTRAINT fk_video_affiliate_item_belongings_item_id FOREIGN KEY (affiliate_item_id) REFERENCES affiliate_items (id)
);