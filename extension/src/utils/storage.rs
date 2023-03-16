use anyhow::{Result, Context, anyhow};
pub use signer::storage::*;
pub use web_sys::Storage;

#[derive(Default)]
pub struct LocalStorage {}

fn get_storage() -> Result<Storage> {
    web_sys::window()
    .context("Window not available")
    .and_then(|window| window
        .local_storage()
        .ok()
        .flatten()
        .context("localStorage not available"))
}

impl Store for LocalStorage {

    fn get_item(&self, key: &str) -> Result<String> {
        let storage = get_storage()?;
        
        storage
            .get_item(&key)
            .ok()
            .flatten()
            .context("Error while getting item from storage")
    }

    fn set_item(&self, key: &str, data: &str) -> Result<()> {
        let storage = get_storage()?;

        storage
            .set_item(&key, data)
            .map_err(|_| anyhow!("Error setting data in storage"))
    }
}