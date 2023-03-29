use anyhow::{anyhow, Context, Result};
use bitcoin::consensus::serialize;
use bitcoin::psbt::Input;
use bitcoin::secp256k1::ecdsa::Signature;
use bitcoin::secp256k1::{Message, Secp256k1};
use bitcoin::util::bip32::ExtendedPrivKey;
use bitcoin::util::psbt::PartiallySignedTransaction;
use bitcoin::util::sighash::SighashCache;
use bitcoin::{EcdsaSig, EcdsaSighashType, PublicKey};
use std::str::FromStr;

use crate::utils::base64::to_base64;
use crate::wallet::Wallet;

fn set_sighash_type(signature: Signature, input: &Input) -> EcdsaSig {
    let sighash_type = get_sighash_type(input);
    let mut sig = EcdsaSig::sighash_all(signature);
    sig.hash_ty = sighash_type;
    sig
}

fn get_sighash_type(input: &Input) -> EcdsaSighashType {
    input
        .sighash_type
        .and_then(|t| t.ecdsa_hash_ty().ok())
        .unwrap_or(EcdsaSighashType::All)
}

fn sign_psbt(
    mut psbt: PartiallySignedTransaction,
    xprv: ExtendedPrivKey,
) -> Result<PartiallySignedTransaction> {
    let secp = Secp256k1::new();

    // https://github.com/bitcoin/bips/blob/master/bip-0174.mediawiki#user-content-Signer

    for (index, input) in psbt.inputs.iter_mut().enumerate() {
        let witness_script = input
            .witness_script
            .as_ref()
            .context("Missing witness script")?;

        let mut sighash_cache = SighashCache::new(&psbt.unsigned_tx);
        let amount = input
            .witness_utxo
            .as_ref()
            .context("Witness utxo not found")?
            .value;
        let sighash = sighash_cache.segwit_signature_hash(
            index,
            witness_script,
            amount,
            get_sighash_type(input),
        )?;

        let mut input_keypairs = Vec::new();

        for (_, (fingerprint, derivation)) in input.bip32_derivation.iter() {
            if fingerprint != &xprv.fingerprint(&secp) {
                continue;
            }
            let parent_xprv = xprv.derive_priv(&secp, &derivation)?;
            input_keypairs.push(parent_xprv.to_keypair(&secp));
        }

        if input_keypairs.is_empty() {
            return Err(anyhow!("No private keys to sign this psbt"));
        }

        for keypair in input_keypairs {
            let message = &Message::from_slice(&sighash)?;
            let signature = secp.sign_ecdsa(message, &keypair.secret_key());
            input.partial_sigs.insert(
                PublicKey::new(keypair.public_key()),
                set_sighash_type(signature, input),
            );

            secp.verify_ecdsa(message, &signature, &keypair.public_key())?;
        }
    }

    Ok(psbt)
}

pub fn decode_psbt_and_sign(psbt_64: &str, wallet: &mut Wallet, password: &str) -> Result<String> {
    let psbt = PartiallySignedTransaction::from_str(psbt_64)?;

    let xprv = wallet.get_xprv(password)?;
    let signed_psbt = sign_psbt(psbt, xprv)?;

    Ok(to_base64(&serialize(&signed_psbt)))
}
