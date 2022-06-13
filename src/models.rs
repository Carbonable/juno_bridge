use crate::schema::config;
use crate::schema::contract;
use crate::schema::transfer;

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "contract"]
pub struct Contract {
    pub id: i32,
    pub title: String,
    pub address: String,
    pub nb_tx: i64,
}

#[derive(Insertable)]
#[table_name = "contract"]
pub struct NewContract {
    pub title: String,
    pub address: String,
}

#[derive(Queryable, Debug, Clone)]
pub struct Config {
    pub id: i32,
    pub lcd_address: String,
    pub wallet_dest: String,
}

#[derive(Insertable)]
#[table_name = "config"]
pub struct NewConfig {
    pub lcd_address: String,
    pub wallet_dest: String,
}

#[derive(Insertable)]
#[table_name = "transfer"]
pub struct NewTransfer {
    pub src: String,
    pub memo: String,
    pub nft_id: String,
    pub contract_id: i32,
}

#[derive(Queryable, Debug)]
pub struct Transfer {
    pub id: i32,
    pub src: String,
    pub memo: String,
    pub nft_id: String,
    pub contract_id: i32,
}
