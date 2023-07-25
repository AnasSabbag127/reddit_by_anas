-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE 
    IF NOT EXISTS users(
        user_id Uuid PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        user_name VARCHAR(20),
        user_email_id VARCHAR(30)
    );

CREATE TABLE 
    IF NOT EXISTS posts(
        id Uuid PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        user_id Uuid,
        post_title VARCHAR(40),
        post_text VARCHAR(200),
        CONSTRAINT fk_post
        FOREIGN KEY(user_id)
        REFERENCES users(user_id)
    );

-- CREATE TABLE 
--     IF NOT EXISTS comments(
--         id Uuid PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
--         user_id Uuid,
--         post_id Uuid,
--         comments VARCHAR(100),
--         CONSTRAINT fk_post
--         FOREIGN KEY(user_id)
--         FOREIGN KEY(post_id)
--         REFERENCES users(user_id)
--         REFERENCES posts(id)
--     );