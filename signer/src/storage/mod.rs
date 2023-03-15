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



#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserStorage {
    pub name: Option<String>,
    pub password: Option<String>,
    pub wallets: Vec<Wallet>
}

pub trait Store {
    fn get_item(&self, key: &str) -> Result<String>;
}

impl UserStorage {
    pub fn read(store: &impl Store) -> Result<UserStorage> {
        store
            .get_item(&StorageKeys::User.to_string())
            .and_then(|value| serde_json::from_str(&value).map_err(|e| anyhow!(e)))
    }

    // pub fn hash_password(&mut self) -> Result<()> {
    //     let salt: [u8; 32] = rand::thread_rng().gen();
    //     let config = Config::default();

    //     let password = argon2::hash_encoded(self.password.unwrap().as_bytes(), &salt, &config)
    //         .context(|e| anyhow!("Failed to hash password: {}", e))?;
    //     self.password = password;

    //     Ok(())
    // }

    // pub fn verify_password(&self, password: &[u8]) -> Result<bool> {
    //     argon2::verify_encoded(&self.password, password)
    //         .map_err(|e| ApiError::new(500, format!("Failed to verify password: {}", e)))
    // }
}