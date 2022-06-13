extern crate contract_watcher;
extern crate diesel;

use clap::Parser;
use contract_watcher::error::BridgeError;
use contract_watcher::service::config::ConfigService;
use contract_watcher::{establish_connection, init};
use log::{error, info};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the contract
    #[clap(short, long)]
    lcd_address: Option<String>,

    #[clap(short, long)]
    wallet_dest: Option<String>,
}

fn main() -> Result<(), BridgeError> {
    let args = Args::parse();

    init();

    let connection = establish_connection();
    let config_service = ConfigService::new(&connection);

    if config_service.get().is_ok() {
        if args.lcd_address.is_some() || args.wallet_dest.is_some() {
            config_service.update(args.lcd_address, args.wallet_dest)?;
        }

        let config = config_service.get()?;
        println!("config {:#?}", config);
    } else if args.lcd_address.is_some() && args.wallet_dest.is_some() {
        config_service.create(args.lcd_address.unwrap(), args.wallet_dest.unwrap())?;
        info!("config created");
    } else {
        error!("no config present, provide a lcd address and a destination wallet");
    }

    Ok(())
}
