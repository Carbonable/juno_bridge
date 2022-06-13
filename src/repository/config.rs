use crate::error::BridgeError;
use crate::models::{Config, NewConfig};
use crate::schema;
use crate::schema::config::dsl::config;
use diesel::prelude::*;
use diesel::{PgConnection, RunQueryDsl};

pub struct ConfigRepository<'a> {
    conn: &'a PgConnection,
}

impl<'a> ConfigRepository<'a> {
    pub fn new(conn: &'a PgConnection) -> Self {
        ConfigRepository { conn }
    }

    pub fn create(&self, lcd_address: String, wallet_dest: String) -> Result<Config, BridgeError> {
        let new_config = NewConfig {
            lcd_address,
            wallet_dest,
        };

        diesel::insert_into(schema::config::table)
            .values(&new_config)
            .get_result(self.conn)
            .map_err(BridgeError::Diesel)
    }

    pub fn get(&self) -> Result<Config, BridgeError> {
        config
            .first::<Config>(self.conn)
            .map_err(BridgeError::Diesel)
    }

    pub fn update(&self, addr: Option<String>, dest: Option<String>) -> Result<(), BridgeError> {
        use crate::schema::config::columns::lcd_address;
        use crate::schema::config::columns::wallet_dest;

        if let Some(a) = addr {
            diesel::update(config)
                .set(lcd_address.eq(a))
                .execute(self.conn)?;
        }

        if let Some(d) = dest {
            diesel::update(config)
                .set(wallet_dest.eq(d))
                .execute(self.conn)?;
        }

        Ok(())
    }
}
