use crate::utils::encryption::{decrypt, encrypt, get_encryption_key, AEAD_NONCE_SIZE_BYTES};
use anyhow::{anyhow, Context, Result};
use bdk::keys::bip39::{Language, Mnemonic, WordCount};
use bdk::keys::{DerivableKey, GeneratedKey};
use bdk::keys::{ExtendedKey, GeneratableKey};
use bdk::miniscript::Segwitv0;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::bip32::{DerivationPath, ExtendedPrivKey, ExtendedPubKey};
use bitcoin::Network;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

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

    pub fn get_xprv(
        &mut self,
        password: &str,
        network: Network,
    ) -> anyhow::Result<ExtendedPrivKey> {
        let salt = self.get_salt();
        let nonce = self.get_nonce();

        let encrypted_secret = match &mut self.secret {
            Some(Secret::Seed(seed)) => seed,
            Some(Secret::XPRV(xprv)) => xprv,
            None => return Err(anyhow!("No secret found")),
        };

        let secret_key = get_encryption_key(&salt, password)?;

        let decrypted_secret = decrypt(secret_key[..].try_into()?, nonce, encrypted_secret)?;

        match self.secret {
            Some(Secret::Seed(_)) => {
                let seed = Mnemonic::parse(decrypted_secret)?;
                let xkey: ExtendedKey = seed.into_extended_key()?;
                let xprv = xkey.into_xprv(network).context("No private key found")?;
                Ok(xprv)
            }
            Some(Secret::XPRV(_)) => {
                let mut xprv =
                    ExtendedPrivKey::from_str(&decrypted_secret).map_err(|e| anyhow!("{}", e))?;
                xprv.network = network;
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

        let decrypted_secret = decrypt(secret_key[..].try_into()?, nonce, encrypted_secret)?;

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

    pub fn derive_xpub(
        &mut self,
        derivation: &str,
        password: &str,
        network: Network,
    ) -> Result<(String, String)> {
        let xprv = self.get_xprv(password, network)?;
        let path = DerivationPath::from_str(derivation)?;
        let secp = Secp256k1::new();
        let derived_xprv = xprv.derive_priv(&secp, &path)?;
        let xpub = ExtendedPubKey::from_priv(&secp, &derived_xprv);

        let master_fingerprint = xprv.fingerprint(&secp).to_string();
        Ok((master_fingerprint, xpub.to_string()))
    }
}

#[test]
fn encrypt_decrypt_seed_success() {
    let mut wallet = Wallet::default();
    let password = "Qwerty123";
    let seed_str = "solar goat auto bachelor chronic input twin depth fork scale divorce fury mushroom column image sauce car public artist announce treat spend jacket physical";
    wallet
        .from_seed_str("Wallet 1", seed_str, password)
        .unwrap();

    let seed = Mnemonic::parse(seed_str).unwrap();
    let xkey: ExtendedKey = seed.into_extended_key().unwrap();
    let xprv = xkey.into_xprv(Network::Bitcoin).unwrap();

    assert_eq!(xprv, wallet.get_xprv(password, Network::Bitcoin).unwrap())
}

#[test]
fn encrypt_decrypt_xpriv_success() {
    let mut wallet = Wallet::default();
    let password = "Qwerty123";
    let xprv_str = "tprv8aXrDeJbcYaRPWkuqtzTMR2Gui4T6A9bwfq6pScH4GSFFzrvXTQ21Fj9fjLzcv4MQxE8yyBtVjrCDn21kbjVvSrghAWU7hGDGQUFZTNADg4";
    let derivation = "m/48'/1'/1'";
    wallet
        .from_xprv_str("Wallet 1", xprv_str, derivation, password)
        .unwrap();

    let mut xprv = ExtendedPrivKey::from_str(xprv_str).unwrap();
    xprv.network = Network::Bitcoin;

    assert_eq!(xprv, wallet.get_xprv(password, Network::Bitcoin).unwrap())
}
