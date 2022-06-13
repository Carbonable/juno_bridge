use crate::error::BridgeError;
use crate::models::Config;
use reqwest::blocking::Response;
use serde::Deserialize;
use serde_json;
use std::thread::sleep;
use std::time::Duration;

#[derive(Default)]
pub struct JunoLCD {
    lcd_address: String,
}

const MAX_RETRY: usize = 5;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct PubKey {
    pub key: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Account {
    pub address: String,
    pub pub_key: Option<PubKey>,
    pub account_number: String,
    pub sequence: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct QueryAccount {
    pub account: Account,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct QueryError {
    pub code: u32,
    pub message: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Pagination {
    pub next_key: Option<String>,
    pub total: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Xfer {
    pub recipient: String,
    pub token_id: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct XferNft {
    pub transfer_nft: Xfer,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TxMessage {
    pub sender: String,
    pub contract: String,
    pub msg: serde_json::Value,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TxBody {
    pub messages: Vec<TxMessage>,
    pub memo: String,
    pub timeout_height: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Tx {
    pub body: TxBody,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct QueryTx {
    pub txs: Vec<Tx>,
    pub pagination: Pagination,
}

impl JunoLCD {
    pub fn new(config: Config) -> Self {
        JunoLCD {
            lcd_address: config.lcd_address,
        }
    }

    fn get(&mut self, endpoint: String) -> Result<Response, BridgeError> {
        for i in 0..MAX_RETRY {
            let addr = self.lcd_address.clone();
            /*            if i % 2 == 0 && self.backup_address.is_some() {
                addr = self.backup_address.clone().unwrap();
            }*/

            // test
            let client = reqwest::blocking::Client::builder()
                .timeout(Duration::from_secs(120))
                .build()
                .map_err(BridgeError::Reqwest)?;
            let request = client.get(format!("{}{}", addr, endpoint.clone())).send();

            if request.is_err() {
                //warn
                log::warn!("access to {}{}", self.lcd_address, endpoint);

                if i < MAX_RETRY {
                    sleep(Duration::from_secs(15));
                } else {
                    log::error!("cannot accesss to {}", endpoint);
                    return Err(BridgeError::ApiGetFailure {
                        lcd: "".to_owned(),
                        path: endpoint,
                    });
                }
            } else {
                return request.map_err(BridgeError::Reqwest);
            }
        }

        // Add notification here.
        Err(BridgeError::ApiGetFailure {
            lcd: self.lcd_address.clone(),
            path: endpoint,
        })
    }

    pub fn get_account(&mut self, address: String) -> Result<QueryAccount, BridgeError> {
        let response = self.get(format!("/cosmos/auth/v1beta1/accounts/{}", address))?;

        let text = response.text()?;

        let data = serde_json::from_str::<QueryAccount>(&text);

        match data {
            Ok(query) => Ok(query),
            Err(_) => {
                let err =
                    serde_json::from_str::<QueryError>(&text).map_err(BridgeError::SerDeJson)?;
                Err(BridgeError::CosmosError {
                    code: err.code,
                    message: err.message,
                })
            }
        }
    }

    pub fn nb_tx(&mut self, address: String) -> Result<i64, BridgeError> {
        let response = self.get(format!("/cosmos/tx/v1beta1/txs?events=wasm._contract_address=%27{}%27&pagination.limit={}&pagination.count_total=true", address, 1))?;
        let text = response.text()?;

        let data = serde_json::from_str::<QueryTx>(&text);

        match data {
            Ok(query) => Ok(query.pagination.total.parse::<i64>().unwrap()),
            Err(e) => {
                log::warn!("cannot deser {:#?}", e);
                let err =
                    serde_json::from_str::<QueryError>(&text).map_err(BridgeError::SerDeJson)?;
                Err(BridgeError::CosmosError {
                    code: err.code,
                    message: err.message,
                })
            }
        }
    }

    pub fn fetch_tx(
        &mut self,
        address: String,
        limit: i64,
        offset: i64,
    ) -> Result<QueryTx, BridgeError> {
        //let response = self.get(format!("/cosmos/tx/v1beta1/txs?events=wasm.action=%27transfer_nft%27&events=wasm._contract_address=%27{}%27&pagination.limit={}&pagination.count_total=true", address, limit))?;
        let url = format!("/cosmos/tx/v1beta1/txs?events=wasm._contract_address=%27{}%27&pagination.limit={}&pagination.count_total=true&pagination.reverse=true&pagination.offset={}", address, limit, offset);
        log::debug!("url {}", url);
        let response = self.get(url)?;
        let text = response.text()?;

        let data = serde_json::from_str::<QueryTx>(&text);

        match data {
            Ok(query) => Ok(query),
            Err(e) => {
                log::warn!("cannot deser {:#?}", e);
                let err =
                    serde_json::from_str::<QueryError>(&text).map_err(BridgeError::SerDeJson)?;
                Err(BridgeError::CosmosError {
                    code: err.code,
                    message: err.message,
                })
            }
        }
    }
}
