use crate::error::BridgeError;
use crate::models::{Contract, NewContract};
use crate::schema;
use crate::schema::contract::columns::nb_tx;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, TextExpressionMethods};

pub struct ContractRepository<'a> {
    conn: &'a PgConnection,
}

impl<'a> ContractRepository<'a> {
    pub fn new(conn: &'a PgConnection) -> Self {
        ContractRepository { conn }
    }

    pub fn create(&self, title: String, address: String) -> Result<Contract, BridgeError> {
        let new_contract = NewContract { title, address };

        diesel::insert_into(schema::contract::table)
            .values(&new_contract)
            .get_result(self.conn)
            .map_err(BridgeError::Diesel)
    }

    pub fn update_tx(&self, contract: Contract, tx: i64) -> Result<(), BridgeError> {
        diesel::update(&contract)
            .set(nb_tx.eq(tx))
            .execute(self.conn)
            .map_err(BridgeError::Diesel)?;

        Ok(())
    }

    pub fn remove(&self, contract_name: String) -> Result<(), BridgeError> {
        use crate::schema::contract::dsl::*;
        diesel::delete(contract.filter(title.like(contract_name)))
            .execute(self.conn)
            .map_err(BridgeError::Diesel)?;

        Ok(())
    }
}
