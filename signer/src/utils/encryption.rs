use aes_gcm::{aead::AeadInPlace, Aes256Gcm, Key, KeyInit, Nonce};
use anyhow::{Result, anyhow};

use std::str;

use super::base64::{to_base64, from_base64};


/// Represents `N_k` from RFC9180.
/// <https://www.rfc-editor.org/rfc/rfc9180.html#name-cryptographic-dependencies>
pub(crate) const AEAD_ALGORITHM_KEY_SIZE_BYTES: usize = 32;
/// Represents `N_n` from RFC9180.
/// <https://www.rfc-editor.org/rfc/rfc9180.html#name-cryptographic-dependencies>
pub(crate) const AEAD_NONCE_SIZE_BYTES: usize = 12;

fn argon2_config<'a>() -> argon2::Config<'a> {
    return argon2::Config {
        variant: argon2::Variant::Argon2id,
        hash_length: 32,
        lanes: 8,
        mem_cost: 16 * 1024,
        time_cost: 8,
        ..Default::default()
    };
}

pub fn get_encryption_key(salt: &[u8; 32], password: &str) -> Result<Vec<u8>> {
    let argon2_config = argon2_config();
    let password = argon2::hash_raw(password.as_bytes(), salt, &argon2_config)?;
    Ok(password)
}

pub fn encrypt(secret_key: [u8; AEAD_ALGORITHM_KEY_SIZE_BYTES], nonce: [u8; AEAD_NONCE_SIZE_BYTES], decrypted_data: &str) -> Result<String> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&secret_key));
    
    let mut ciphertext = Vec::from(decrypted_data);
    cipher
        .encrypt_in_place(Nonce::from_slice(&nonce), &[], &mut ciphertext)
        .map_err(|error| anyhow!("couldn't encrypt data: {}", error))?;

    Ok(to_base64(&ciphertext))
}

pub fn decrypt(secret_key: [u8; AEAD_ALGORITHM_KEY_SIZE_BYTES], nonce: [u8; AEAD_NONCE_SIZE_BYTES], encrypted_data: &str) -> Result<String> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&secret_key[..AEAD_ALGORITHM_KEY_SIZE_BYTES]));
    
    let mut plaintext = from_base64(encrypted_data)?;
    cipher
        .decrypt_in_place(Nonce::from_slice(&nonce), &[], &mut plaintext)
        .map_err(|error| anyhow!("couldn't decrypt data: {}", error))?;

    str::from_utf8(&plaintext)
        .map(|s| s.to_string())
        .map_err(|e| anyhow!("{e}"))
}

#[cfg(test)]
mod test {
    use crate::utils::{base64::to_base64, encryption::{get_encryption_key, decrypt}};
    use super::encrypt;
    
    #[test]
    fn get_encryption_key_success() {
        let salt: Vec<u8> = (0..32).collect(); 
        let key = get_encryption_key(salt[..].try_into().unwrap(), "Qwerty123").unwrap();
        assert_eq!(to_base64(&key), "Jl9nM4d/UTcsT9EQFd6slgxhFnrOYiZK/Ze5kHW6450=")
    }

    #[test]
    fn encrypt_success() {
        let salt: Vec<u8> = (0..32).collect(); 
        let secret_key = get_encryption_key(salt[..].try_into().unwrap(), "Qwerty123").unwrap();
        let nonce: Vec<u8> = (0..12).collect();

        let data = "Hello World!";
        let encrypted = encrypt(secret_key[..].try_into().unwrap(), nonce[..].try_into().unwrap(), data).unwrap();
        assert_eq!(encrypted, "Pmfe/4uMNFUlFooCRMffz9V9aR066/z4XjcyQw==")
    }

    #[test]
    fn decrypt_success() {
        let salt: Vec<u8> = (0..32).collect(); 
        let secret_key = get_encryption_key(salt[..].try_into().unwrap(), "Qwerty123").unwrap();
        let nonce: Vec<u8> = (0..12).collect();

        let data = "Pmfe/4uMNFUlFooCRMffz9V9aR066/z4XjcyQw==";
        let encrypted = decrypt(secret_key[..].try_into().unwrap(), nonce[..].try_into().unwrap(), data).unwrap();
        assert_eq!(encrypted, "Hello World!")
    }

}