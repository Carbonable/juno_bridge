use crate::error::BridgeError;
use crate::models::{NewTransfer, Transfer};
use crate::schema;
use diesel::{PgConnection, RunQueryDsl};

pub struct TransferRepository<'a> {
    conn: &'a PgConnection,
}

impl<'a> TransferRepository<'a> {
    pub fn new(conn: &'a PgConnection) -> Self {
        TransferRepository { conn }
    }

    pub fn create(
        &self,
        src: String,
        memo: String,
        nft_id: String,
        contract_id: i32,
    ) -> Result<Transfer, BridgeError> {
        let new_tx = NewTransfer {
            src,
            memo,
            nft_id,
            contract_id,
        };

        diesel::insert_into(schema::transfer::table)
            .values(&new_tx)
            .get_result(self.conn)
            .map_err(BridgeError::Diesel)
    }
}
