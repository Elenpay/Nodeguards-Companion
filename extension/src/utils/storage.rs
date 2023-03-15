use anyhow::{Result, Context};
pub use signer::storage::*;
pub use web_sys::Storage;

#[derive(Default)]
pub struct LocalStorage {}

impl Store for LocalStorage {
    fn get_item(&self, key: &str) -> Result<String> {
        let storage = web_sys::window()
            .context("Window not available")
            .and_then(|window| window
                .local_storage()
                .ok()
                .flatten()
                .context("localStorage not available"))?;
        
        storage
            .get_item(&key)
            .ok()
            .flatten()
            .context("Error while getting item from storage")
    }
}