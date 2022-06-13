table! {
    config (id) {
        id -> Int4,
        lcd_address -> Varchar,
        wallet_dest -> Varchar,
    }
}

table! {
    contract (id) {
        id -> Int4,
        title -> Varchar,
        address -> Varchar,
        nb_tx -> Int8,
    }
}

table! {
    transfer (id) {
        id -> Int4,
        src -> Varchar,
        memo -> Varchar,
        nft_id -> Varchar,
        contract_id -> Int4,
    }
}

joinable!(transfer -> contract (contract_id));

allow_tables_to_appear_in_same_query!(
    config,
    contract,
    transfer,
);
