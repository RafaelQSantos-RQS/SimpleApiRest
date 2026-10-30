CREATE TABLE IF NOT EXISTS usuarios (
    id              TEXT PRIMARY KEY NOT NULL,
    email           TEXT NOT NULL UNIQUE,
    nome            TEXT NOT NULL,
    senha_hash      TEXT NOT NULL,
    criado_em       TEXT NOT NULL
);