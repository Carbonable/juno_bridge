-- This file should undo anything in `up.sql`
CREATE TABLE transfer (
                           id SERIAL PRIMARY KEY,
                           src VARCHAR NOT NULL,
                           memo VARCHAR NOT NULL,
                           nft_id VARCHAR NOT NULL,
                           contract_id SERIAL,
                           CONSTRAINT fk_contract
                               FOREIGN KEY(contract_id)
                                   REFERENCES contract(id)
)