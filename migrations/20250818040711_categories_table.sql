-- Add migration script here
CREATE TABLE IF NOT EXISTS categories (
    id bigint NOT NULL GENERATED ALWAYS AS IDENTITY,
    "uuid" UUID DEFAULT gen_random_uuid() NULL,
    "name" VARCHAR(255) NOT NULL,
    "description" VARCHAR(255) NULL,
    "created_at" TIMESTAMP DEFAULT now() NULL,
    "updated_at" TIMESTAMP NULL,
    "deleted_at" TIMESTAMP NULL,
    CONSTRAINT categories_pk PRIMARY KEY (id)
);

