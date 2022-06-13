use crate::error::BridgeError;
use crate::models::Config;
use crate::repository::config::ConfigRepository;
use diesel::PgConnection;

pub struct ConfigService<'a> {
    repo: ConfigRepository<'a>,
}

impl<'a> ConfigService<'a> {
    pub fn new(connection: &'a PgConnection) -> Self {
        ConfigService {
            repo: ConfigRepository::new(connection),
        }
    }

    pub fn create(&self, lcd: String, juno_address: String) -> Result<Config, BridgeError> {
        self.repo.create(lcd, juno_address)
    }

    pub fn get(&self) -> Result<Config, BridgeError> {
        self.repo.get()
    }

    pub fn update(
        &self,
        lcd: Option<String>,
        juno_address: Option<String>,
    ) -> Result<(), BridgeError> {
        self.repo.update(lcd, juno_address)
    }
}
