use bdk::keys::{ExtendedKey, bip39::Mnemonic};
use bitcoin::{util::bip32::{ExtendedPrivKey, DerivationPath}, Network};
use bdk::keys::DerivableKey;
use anyhow::{Result, Context, anyhow};
use rand::Rng;
use serde::{Serialize, Deserialize};

use crate::utils::encryption::{get_encryption_key, encrypt, decrypt, AEAD_NONCE_SIZE_BYTES};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Wallet {
    pub name: String,
    salt: Option<[u8; 32]>,
    nonce: Option<[u8; AEAD_NONCE_SIZE_BYTES]>,
    xprv: Option<String>,
    pub derivation: Option<DerivationPath>
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
    
    pub(crate) fn encrypt_xprv(&mut self, password: &str, xprv: ExtendedPrivKey) -> anyhow::Result<String> {
        let secret_key = get_encryption_key(&self.get_salt(), password)?;
        let xprv_decrypted = xprv.to_string();

        encrypt(secret_key[..].try_into()?, self.get_nonce(), &xprv_decrypted)
    }

    pub(crate) fn get_xprv(&mut self, password: &str) -> anyhow::Result<ExtendedPrivKey> {
        let secret_key = get_encryption_key(&self.get_salt(), password)?;
        let xprv_encrypted = self.xprv.clone().context("No xprv found")?; 
        
        let xprv_decrypted = decrypt(secret_key[..].try_into()?, self.get_nonce(), &xprv_encrypted)?; 
        ExtendedPrivKey::decode(&xprv_decrypted.as_bytes()).map_err(|e| anyhow!("{}", e))
    }

    pub fn from_mnemonic_str(&mut self, name: &str, mnemonic: &str, password: &str) -> Result<()> {
        let mnemonic = Mnemonic::parse(mnemonic).unwrap();
        let xkey: ExtendedKey = mnemonic.into_extended_key().unwrap();
        let xprv = xkey.into_xprv(Network::Regtest).context("No private key found").unwrap();
        
        let encrypted_xprv = self.encrypt_xprv(password, xprv)?;

        self.name = name.to_string();
        self.xprv = Some(encrypted_xprv);
        self.derivation = Some(DerivationPath::default());

        Ok(())
    }
}

    
#[test]
fn encrypt_xpriv_success() {    
    let mut wallet = Wallet::default();
    let password = "Qwerty123";
    let mnemonic_str = "solar goat auto bachelor chronic input twin depth fork scale divorce fury mushroom column image sauce car public artist announce treat spend jacket physical";
    wallet.from_mnemonic_str("Wallet 1", mnemonic_str, password).unwrap();
    
    let mnemonic = Mnemonic::parse(mnemonic_str).unwrap();
    let xkey: ExtendedKey = mnemonic.into_extended_key().unwrap();
    let xprv = xkey.into_xprv(Network::Regtest).context("No private key found").unwrap();

    assert_eq!(xprv, wallet.get_xprv(password).unwrap())
}