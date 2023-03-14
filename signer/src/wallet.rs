use bdk::keys::{ExtendedKey, bip39::Mnemonic};
use bitcoin::{util::bip32::{ExtendedPrivKey}, Network};
use bdk::keys::DerivableKey;

use anyhow::{Context};

pub struct Wallet {
    pub xprv: ExtendedPrivKey
}

impl Wallet {
    pub fn from_mnemonic_str(mnemonic: &str) -> Self {
        let mnemonic = Mnemonic::parse(mnemonic).unwrap();
        let xkey: ExtendedKey = mnemonic.into_extended_key().unwrap();
        let xprv = xkey.into_xprv(Network::Regtest).context("No private key found").unwrap();
        Self {
            xprv
        }
    }
}