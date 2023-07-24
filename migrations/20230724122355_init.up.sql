-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid::ossp";

CREATE TABLE 
    IF NOT EXISTS User(
        user_id Uuid PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        user_name VARCHAR(20),
        user_email_id VARCHAR(30)
    );
