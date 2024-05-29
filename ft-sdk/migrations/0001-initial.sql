CREATE TABLE IF NOT EXISTS fastn_email_queue
(
    id           INTEGER PRIMARY KEY,
    from_address TEXT NOT NULL,
    reply_to     TEXT NULL,
    -- to_address, cc_address, bcc_address contains comma separated email with
    -- names https://users.rust-lang.org/t/80813/11
    -- Alice <test1@gmail.com>, Bob <test2@ocr-inc.com>
    to_address   TEXT NOT NULL,
    cc_address   TEXT NULL,
    bcc_address  TEXT NULL,
    subject      TEXT NOT NULL,
    body_text    TEXT NOT NULL,
    body_html    TEXT NOT NULL,
    retry_count  INTEGER NOT NULL DEFAULT 0,
    created_at   INTEGER NOT NULL,
    sent_at      INTEGER NOT NULL,
    -- mkind is any string, used for product analytics etc
    mkind        TEXT NOT NULL,
    -- status: pending, sent, failed. sent and failed items may removed from
    -- the queue every so often
    status       TEXT NOT NULL
) STRICT;


CREATE TABLE IF NOT EXISTS fastn_user
(
    id       INTEGER PRIMARY KEY,
    name     TEXT NULL,
    identity TEXT UNIQUE NOT NULL,
    -- this stores ft_sdk::auth::UserData
    data     TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
) STRICT;


CREATE TABLE IF NOT EXISTS fastn_session
(
    id   TEXT PRIMARY KEY,
    uid  INTEGER NULL,
    data TEXT, -- this is the session data only
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,

    CONSTRAINT fk_fastn_user
    FOREIGN KEY (uid)
    REFERENCES fastn_user (id)
) STRICT;