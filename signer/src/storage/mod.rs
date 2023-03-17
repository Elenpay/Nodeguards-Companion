use std::fmt;
use anyhow::{Result, anyhow, Context};
use serde::{Serialize, Deserialize};
use argon2::Config;
use rand::Rng;

pub use crate::wallet::Wallet;

pub enum StorageKeys {
    User
}

impl fmt::Display for StorageKeys {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StorageKeys::User => write!(f, "user"),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct UserStorage {
    #[serde(skip_serializing, skip_deserializing)]
    store: Option<Box<dyn Store>>,
    pub name: Option<String>,
    password: Option<String>,
    pub wallets: Vec<Wallet>
}

pub trait Store {
    fn get_item(&self, key: &str) -> Result<String>;
    fn set_item(&self, key: &str, data: &str) -> Result<()>;
}

impl UserStorage {
    pub fn read(store: impl Store + 'static) -> UserStorage {
        let mut user_storage: UserStorage = store
            .get_item(&StorageKeys::User.to_string())
            .and_then(|value| serde_json::from_str(&value).map_err(|e| anyhow!("{}", e)))
            .unwrap_or_default();

        user_storage.store = Some(Box::new(store));
        user_storage
    }

    pub fn save(&mut self) -> Result<()> {
        let data = serde_json::to_string(&self)?;
        self.store
            .as_mut()
            .context("Store not found")?
            .set_item(&StorageKeys::User.to_string(), &data)
    }

    pub fn has_password(&self) -> bool {
        self.password.is_some()
    }

    pub fn set_password(&mut self, password: &str) -> Result<()> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();

        let password = argon2::hash_encoded(password.as_bytes(), &salt, &config)
            .map_err(|e| anyhow!("Failed to hash password: {}", e))?;
        self.password = Some(password);

        Ok(())
    }

    pub fn verify_password(&self, password: &[u8]) -> Result<bool> {
        if self.password.is_none() {
            return Ok(false);
        }
        argon2::verify_encoded(&self.password.as_ref().unwrap(), password)
            .map_err(|e| anyhow!("Failed to verify password: {}", e))
    }
}