-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id bigint NOT NULL GENERATED ALWAYS AS IDENTITY,
    "uuid" UUID DEFAULT gen_random_uuid() NULL,
    "username" VARCHAR(255) NOT NULL,
    "email" VARCHAR(255) NOT NULL,
    "password" VARCHAR(255) NOT NULL,
    "role" VARCHAR(10) NOT NULL,
    "is_active" BOOLEAN NULL DEFAULT false,
    "created_at" TIMESTAMP DEFAULT now() NULL,
    "updated_at" TIMESTAMP NULL,
    "deleted_at" TIMESTAMP NULL,
    CONSTRAINT users_pk PRIMARY KEY (id)
);
