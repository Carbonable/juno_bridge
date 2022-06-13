use crate::error::BridgeError;
use crate::models::Contract;
use crate::repository::contract::ContractRepository;
use diesel::PgConnection;

pub struct ContractService<'a> {
    repo: ContractRepository<'a>,
}

impl<'a> ContractService<'a> {
    pub fn new(connection: &'a PgConnection) -> Self {
        ContractService {
            repo: ContractRepository::new(connection),
        }
    }

    pub fn create(&self, title: String, address: String) -> Result<Contract, BridgeError> {
        self.repo.create(title, address)
    }

    pub fn update_tx(&self, contract: Contract, tx: i64) -> Result<(), BridgeError> {
        self.repo.update_tx(contract, tx)
    }

    pub fn remove(&self, contract_name: String) -> Result<(), BridgeError> {
        self.repo.remove(contract_name)
    }
}
