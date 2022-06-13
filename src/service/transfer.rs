use crate::error::BridgeError;
use crate::models::Transfer;
use crate::repository::transfer::TransferRepository;
use diesel::PgConnection;

pub struct TransferService<'a> {
    repo: TransferRepository<'a>,
}

impl<'a> TransferService<'a> {
    pub fn new(connection: &'a PgConnection) -> Self {
        TransferService {
            repo: TransferRepository::new(connection),
        }
    }

    pub fn create(
        &self,
        src: String,
        memo: String,
        nft_id: String,
        contract_id: i32,
    ) -> Result<Transfer, BridgeError> {
        self.repo.create(src, memo, nft_id, contract_id)
    }
}
