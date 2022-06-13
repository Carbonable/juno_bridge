extern crate contract_watcher;
extern crate diesel;

use self::contract_watcher::*;
use self::diesel::prelude::*;
use self::models::*;

use clap::Parser;
use contract_watcher::error::BridgeError;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {}

fn main() -> Result<(), BridgeError> {
    use contract_watcher::schema::contract::dsl::*;
    init();

    Args::parse();

    let connection = establish_connection();

    let results = contract
        .limit(10)
        .load::<Contract>(&connection)
        .expect("Error loading contracts");

    for ctat in results {
        println!("{:#?}", ctat);
    }
    Ok(())
}
