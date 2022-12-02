CREATE TABLE IF NOT EXISTS links
(
    id SERIAL PRIMARY KEY,
    resolved_title
    TEXT NOT NULL, resolved_url TEXT NOT NULL, resolved_status INT NOT NULL, added_on TEXT NOT NULL, item_id TEXT NOT NULL UNIQUE);

CREATE TABLE IF NOT EXISTS users
(
    id SERIAL PRIMARY KEY,
    username
    VARCHAR
(
    255
) UNIQUE NOT NULL, email VARCHAR
(
    255
) NOT NULL, created_on TEXT NOT NULL);

CREATE TABLE IF NOT EXISTS users
(
    id SERIAL PRIMARY KEY,
    username
    VARCHAR
(
    255
) UNIQUE NOT NULL, email VARCHAR
(
    255
) NOT NULL, created_on TEXT NOT NULL);

CREATE TABLE IF NOT EXISTS settings
(
    id SERIAL PRIMARY KEY,
    key
    VARCHAR
(
    255
) NOT NULL,
    value
    TEXT
    );

INSERT INTO settings (key, value)
VALUES ('debug', 'true'),
       ('last_retrieval', NULL),
       ('batch_schedule', '0 0 0/6 * * *'),
       ('enable_mail_notification', TRUE);