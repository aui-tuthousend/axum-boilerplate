-- Add migration script here
CREATE TABLE IF NOT EXISTS products (
    id bigint NOT NULL GENERATED ALWAYS AS IDENTITY,
    "uuid" UUID DEFAULT gen_random_uuid() NULL,
    "name" VARCHAR(255) NOT NULL,
    "description" VARCHAR(255) NULL,
    "category_id" bigint NOT NULL,
    "image_url" VARCHAR(255) NULL,
    "price" INT NOT NULL,
    "stock" INT NOT NULL,
    "created_at" TIMESTAMP DEFAULT now() NULL,
    "updated_at" TIMESTAMP NULL,
    "deleted_at" TIMESTAMP NULL,
    
    CONSTRAINT products_pk PRIMARY KEY (id),
    CONSTRAINT products_category_fk FOREIGN KEY (category_id) REFERENCES categories(id)
);
