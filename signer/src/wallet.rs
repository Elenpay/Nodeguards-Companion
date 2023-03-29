use std::str::FromStr;

use crate::utils::encryption::{decrypt, encrypt, get_encryption_key, AEAD_NONCE_SIZE_BYTES};
use crate::NETWORK;
use anyhow::{anyhow, Context, Result};
use bdk::keys::bip39::{Language, Mnemonic, WordCount};
use bdk::keys::{DerivableKey, GeneratedKey};
use bdk::keys::{ExtendedKey, GeneratableKey};
use bdk::miniscript::Segwitv0;
use bitcoin::util::bip32::{DerivationPath, ExtendedPrivKey};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Secret {
    Seed(String),
    XPRV(String),
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Wallet {
    pub name: String,
    salt: Option<[u8; 32]>,
    nonce: Option<[u8; AEAD_NONCE_SIZE_BYTES]>,
    secret: Option<Secret>,
    pub derivation: DerivationPath,
}

impl Wallet {
    fn get_salt(&mut self) -> [u8; 32] {
        if self.salt.is_none() {
            self.salt = Some(rand::thread_rng().gen());
        }

        self.salt.unwrap()
    }

    fn get_nonce(&mut self) -> [u8; AEAD_NONCE_SIZE_BYTES] {
        if self.nonce.is_none() {
            self.nonce = Some(rand::thread_rng().gen());
        }
        self.nonce.unwrap()
    }

    pub(crate) fn encrypt_secret(
        &mut self,
        password: &str,
        decrypted_secret: String,
    ) -> anyhow::Result<String> {
        let secret_key = get_encryption_key(&self.get_salt(), password)?;

        encrypt(
            secret_key[..].try_into()?,
            self.get_nonce(),
            &decrypted_secret,
        )
    }

    pub fn get_xprv(&mut self, password: &str) -> anyhow::Result<ExtendedPrivKey> {
        let salt = self.get_salt();
        let nonce = self.get_nonce();

        let encrypted_secret = match &mut self.secret {
            Some(Secret::Seed(seed)) => seed,
            Some(Secret::XPRV(xprv)) => xprv,
            None => return Err(anyhow!("No secret found")),
        };

        let secret_key = get_encryption_key(&salt, password)?;

        let decrypted_secret = decrypt(secret_key[..].try_into()?, nonce, &encrypted_secret)?;

        match self.secret {
            Some(Secret::Seed(_)) => {
                let seed = Mnemonic::parse(decrypted_secret)?;
                let xkey: ExtendedKey = seed.into_extended_key()?;
                let xprv = xkey.into_xprv(NETWORK).context("No private key found")?;
                Ok(xprv)
            }
            Some(Secret::XPRV(_)) => {
                let mut xprv =
                    ExtendedPrivKey::from_str(&decrypted_secret).map_err(|e| anyhow!("{}", e))?;
                xprv.network = NETWORK;
                Ok(xprv)
            }
            None => unreachable!(),
        }
    }

    pub fn reveal_secret(&mut self, password: &str) -> anyhow::Result<String> {
        let salt = self.get_salt();
        let nonce = self.get_nonce();

        let encrypted_secret = match &mut self.secret {
            Some(Secret::Seed(seed)) => seed,
            Some(Secret::XPRV(xprv)) => xprv,
            None => return Err(anyhow!("No secret found")),
        };

        let secret_key = get_encryption_key(&salt, password)?;

        let decrypted_secret = decrypt(secret_key[..].try_into()?, nonce, &encrypted_secret)?;

        Ok(decrypted_secret)
    }

    pub fn from_seed_str(&mut self, name: &str, seed: &str, password: &str) -> Result<()> {
        let encrypted_seed = self.encrypt_secret(password, seed.to_string())?;

        self.name = name.to_string();
        self.secret = Some(Secret::Seed(encrypted_seed));
        self.derivation = DerivationPath::default();

        Ok(())
    }

    pub fn from_xprv_str(
        &mut self,
        name: &str,
        xprv: &str,
        derivation: &str,
        password: &str,
    ) -> Result<()> {
        let encrypted_xprv = self.encrypt_secret(password, xprv.to_string())?;

        self.name = name.to_string();
        self.secret = Some(Secret::XPRV(encrypted_xprv));
        self.derivation =
            DerivationPath::from_str(derivation).context("Error parsing derivation path")?;

        Ok(())
    }

    pub fn validate(xprv: &str, derivation: &str) -> Result<()> {
        ExtendedPrivKey::from_str(xprv)?;
        DerivationPath::from_str(derivation)?;
        Ok(())
    }

    pub fn generate_seed() -> Result<String> {
        let seed: GeneratedKey<_, Segwitv0> =
            Mnemonic::generate((WordCount::Words24, Language::English))
                .map_err(|_| anyhow!("Error while generating seed"))?;
        Ok(seed.to_string())
    }
}

#[test]
fn encrypt_decrypt_xpriv_success() {
    let mut wallet = Wallet::default();
    let password = "Qwerty123";
    let seed_str = "solar goat auto bachelor chronic input twin depth fork scale divorce fury mushroom column image sauce car public artist announce treat spend jacket physical";
    wallet
        .from_seed_str("Wallet 1", seed_str, password)
        .unwrap();

    let seed = Mnemonic::parse(seed_str).unwrap();
    let xkey: ExtendedKey = seed.into_extended_key().unwrap();
    let xprv = xkey.into_xprv(NETWORK).unwrap();

    assert_eq!(xprv, wallet.get_xprv(password).unwrap())
}
