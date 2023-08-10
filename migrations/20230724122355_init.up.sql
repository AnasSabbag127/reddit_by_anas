-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- CREATE TABLE 
--     IF NOT EXISTS users(
--         id Uuid PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
--         user_name VARCHAR(20),
--         user_email_id VARCHAR(30)
--     );

-- CREATE TABLE 
--     IF NOT EXISTS posts(
--         id Uuid PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
--         user_id Uuid,
--         post_title VARCHAR(40),
--         post_text VARCHAR(200),
--         CONSTRAINT fk_post
--         FOREIGN KEY(user_id)
--         REFERENCES users(user_id)
--     );

-- CREATE TABLE IF NOT EXISTS comments(
--     id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
--     user_id UUID,
--     post_id UUID,
--     comment VARCHAR(100) NOT NULL,
--     CONSTRAINT fk_user
--         FOREIGN KEY(user_id)
--         REFERENCES users(user_id),
--     CONSTRAINT fk_post
--         FOREIGN KEY(post_id) 
--         REFERENCES posts(id)
-- );


-- //for authentication example
-- CREATE TABLE IF NOT EXISTS articles (
--   id SERIAL PRIMARY KEY,
--   title VARCHAR(55),
--   content VARCHAR(200),
--   published_by INT,
--   CONSTRAINT fk_articles_users_new 
--   FOREIGN KEY (published_by) 
--   REFERENCES users (id),
-- );

-- CREATE TABLE IF NOT EXISTS users_new (
--   id SERIAL PRIMARY KEY,
--   username VARCHAR(255),
--   password VARCHAR(255),
-- );



CREATE TABLE IF NOT EXISTS account_user (
  id Uuid PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
  username VARCHAR(255) NOT NULL UNIQUE,
  password VARCHAR(255) NOT NULL UNIQUE
);


CREATE TABLE 
    IF NOT EXISTS posts(
        id Uuid PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        user_id Uuid,
        post_title VARCHAR(40) NOT NULL UNIQUE,
        post_text VARCHAR(200) NOT NULL,
        CONSTRAINT fk_post
        FOREIGN KEY(user_id)
        REFERENCES account_user(id)
    );

CREATE TABLE IF NOT EXISTS comments(
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    user_id UUID,
    post_id UUID,
    comment VARCHAR(100) NOT NULL,
    reply_on_comment UUID,
    CONSTRAINT fk_user
        FOREIGN KEY(user_id)
        REFERENCES account_user(id),
    CONSTRAINT fk_post
        FOREIGN KEY(post_id) 
        REFERENCES posts(id)
);

CREATE TABLE 
    IF NOT EXISTS followers(
        user_id UUID NOT NULL,
        follower_id UUID NOT NULL,
        PRIMARY KEY(user_id,follower_id),
        FOREIGN KEY (user_id) REFERENCES account_user(id),
        FOREIGN KEY (follow) REFERENCES account_user(id) 
    );

CREATE TABLE 
    IF NOT EXISTS posts_image(
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        post_id UUID NOT NULL,
        image NOT NULL BYTEA,
        PRIMARY KEY(id),
        FOREIGN KEY (post_id) REFERENCES posts(id)
    );
