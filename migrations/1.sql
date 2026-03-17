-- Active: 1768567521297@@127.0.0.1@5432@rchat
CREATE TABLE "user" (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL
);