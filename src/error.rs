use diesel::result::Error;
use reqwest;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BridgeError {
    #[error("{0}")]
    Diesel(#[from] Error),

    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("{0}")]
    SerDeJson(#[from] serde_json::Error),

    #[error("No config Present")]
    NoConfig {},

    #[error("Api Get Failure lcd {lcd:?} path {path:?}")]
    ApiGetFailure { lcd: String, path: String },

    #[error("CosmosSDK error {code:?} message {message:?}")]
    CosmosError { code: u32, message: String },
}
