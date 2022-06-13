extern crate contract_watcher;
extern crate diesel;

use contract_watcher::*;

use clap::Parser;
use contract_watcher::error::BridgeError;
use contract_watcher::service::contract;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the contract
    #[clap(short, long)]
    name: String,
}

fn main() -> Result<(), BridgeError> {
    init();

    let args = Args::parse();

    let connection = establish_connection();

    let contract_service = contract::ContractService::new(&connection);
    contract_service.remove(args.name)?;

    Ok(())
}
