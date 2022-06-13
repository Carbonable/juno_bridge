extern crate contract_watcher;
extern crate diesel;

use self::contract_watcher::*;
use self::diesel::prelude::*;
use self::models::*;
use log::error;
use std::cmp;

use clap::Parser;
use contract_watcher::error::BridgeError;
use contract_watcher::service::config::ConfigService;
use contract_watcher::service::contract::ContractService;
use contract_watcher::service::juno_lcd::{JunoLCD, XferNft};
use contract_watcher::service::transfer::TransferService;
use cronjob::CronJob;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {}

const PAGINATION: i64 = 10;

fn watch_contract(
    contract: Contract,
    lcd: &mut JunoLCD,
    contract_service: &ContractService,
    transfer_service: &TransferService,
    config: Config,
) -> Result<(), BridgeError> {
    log::info!("\t=> watching {} => {}", contract.title, contract.address);

    let txs = lcd.nb_tx(contract.address.clone())?;

    log::debug!("\t=> {} txs for contract {}", txs, contract.id);
    let mut transaction_number = contract.nb_tx;
    while transaction_number < txs {
        let take = cmp::min(txs - transaction_number, PAGINATION);
        let txs = lcd.fetch_tx(contract.address.clone(), take, transaction_number)?;
        for tx in txs.txs {
            transaction_number += 1;
            for msg in tx.body.messages {
                let obj = msg.msg.as_object().unwrap();
                for (k, _) in obj {
                    if k == "transfer_nft" {
                        let xfr = serde_json::from_str::<XferNft>(msg.msg.to_string().as_str())?;

                        if xfr.transfer_nft.recipient == config.wallet_dest {
                            log::info!("\t\tnft bridged {}", xfr.transfer_nft.token_id);
                            transfer_service.create(
                                msg.sender.clone(),
                                tx.body.memo.clone(),
                                xfr.transfer_nft.token_id.clone(),
                                contract.id,
                            )?;
                        } else {
                            log::info!(
                                "\t\t=> {} xfer from => {} action => {} to => {} nft {}",
                                transaction_number,
                                msg.sender,
                                k,
                                xfr.transfer_nft.recipient,
                                xfr.transfer_nft.token_id
                            );
                        }
                    } else {
                        log::info!(
                            "\t\t=> {} xfer from => {} action => {}", /*to => {} nft {}*/
                            transaction_number,
                            msg.sender,
                            k /*msg.msg.transfer_nft.recipient, msg.msg.transfer_nft.token_id*/
                        );
                    }
                }
            }
        }
    }

    if transaction_number != contract.nb_tx {
        log::info!(
            "\t=> contract {} update nb_tx to {}",
            contract.id,
            transaction_number
        );
        contract_service.update_tx(contract, transaction_number)?;
    } else {
        log::info!("\t=> no new txs");
    }

    Ok(())
}

fn lookup_task(_name: &str) {
    use contract_watcher::schema::contract::dsl::*;

    let connection = establish_connection();

    let cfg_service = ConfigService::new(&connection);
    let contract_service = ContractService::new(&connection);
    let transfer_service = TransferService::new(&connection);

    let cfg = cfg_service.get();
    if cfg.is_err() {
        error!("no config found please add one first");
        return;
    }
    let cfg = cfg.unwrap();

    let results = contract
        .limit(10)
        .load::<Contract>(&connection)
        .expect("Error loading contracts");

    let mut lcd = JunoLCD::new(cfg.clone());

    log::info!("Watching {} contracts", results.len());
    for ct in results {
        let addr = ct.address.clone();
        match watch_contract(
            ct,
            &mut lcd,
            &contract_service,
            &transfer_service,
            cfg.clone(),
        ) {
            Ok(_) => {}
            Err(e) => {
                log::error!("fail to watch contract: {} err {:?}", addr, e)
            }
        }
    }
}

fn main() {
    let _ = Args::parse();

    init();

    let mut cron = CronJob::new("bridge Cron", lookup_task);
    cron.seconds("0");
    cron.minutes("0,5,10,15,20,25,30,35,40,45,50,55");

    cron.start_job();
}
