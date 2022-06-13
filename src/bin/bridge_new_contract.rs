extern crate contract_watcher;
extern crate diesel;

use contract_watcher::*;

use clap::Parser;
use contract_watcher::error::BridgeError;
use contract_watcher::service::juno_lcd;
use contract_watcher::service::{config, contract};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the contract
    #[clap(short, long)]
    name: String,

    /// Address of the contract
    #[clap(short, long)]
    address: String,
}

fn main() -> Result<(), BridgeError> {
    init();

    let args = Args::parse();

    let connection = establish_connection();

    let config_service = config::ConfigService::new(&connection);
    let contract_service = contract::ContractService::new(&connection);

    if config_service.get().is_err() {
        log::error!("no config found please add one first");
        return Err(BridgeError::NoConfig {});
    }

    let mut lcd = juno_lcd::JunoLCD::new(config_service.get()?);

    let account = lcd.get_account(args.address.clone());

    if let Err(e) = account {
        log::error!("cannot add contract address {} not valid", args.address);
        return Err(e);
    }

    let post = contract_service.create(args.name.clone(), args.address);
    println!("\nSaved contract {} with id {}", args.name, post?.id);

    Ok(())
}
