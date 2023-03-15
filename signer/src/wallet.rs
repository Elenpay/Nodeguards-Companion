use bdk::keys::{ExtendedKey, bip39::Mnemonic};
use bitcoin::{util::bip32::{ExtendedPrivKey, DerivationPath}, Network};
use bdk::keys::DerivableKey;

use anyhow::{Context};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Wallet {
    pub name: String,
    pub xprv: Option<ExtendedPrivKey>,
    pub derivation: Option<DerivationPath>
}

impl Wallet {
    pub fn from_mnemonic_str(name: &str, mnemonic: &str) -> Self {
        let mnemonic = Mnemonic::parse(mnemonic).unwrap();
        let xkey: ExtendedKey = mnemonic.into_extended_key().unwrap();
        let xprv = xkey.into_xprv(Network::Regtest).context("No private key found").unwrap();
        Self {
            name: name.to_string(),
            xprv: Some(xprv),
            derivation: Some(DerivationPath::default()),
        }
    }
}