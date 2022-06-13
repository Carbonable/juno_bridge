CREATE TABLE contract (
                       id SERIAL PRIMARY KEY,
                       title VARCHAR NOT NULL UNIQUE,
                       address VARCHAR NOT NULL UNIQUE,
                       nb_tx BIGINT NOT NULL DEFAULT '0'
)